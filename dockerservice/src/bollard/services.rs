use super::conversions::status_filter_value;
use crate::Result;
use crate::models::{ContainerFilter, ContainerInfo, StateEnum};
use crate::{ContainerService, SystemService};
use async_trait::async_trait;
use bollard::Docker;
use bollard::query_parameters::{ListContainersOptions, ListContainersOptionsBuilder};
use std::collections::HashMap;

pub struct DockerServiceImpl {
    client: Docker,
}

impl DockerServiceImpl {
    pub fn new() -> Result<Self> {
        #[cfg(target_family = "windows")]
        unimplemented!();

        #[cfg(target_family = "unix")]
        let client = Docker::connect_with_local_defaults()?;

        Ok(Self { client })
    }
}

impl SystemService for DockerServiceImpl {
    fn version(&self) -> String {
        self.client.client_version().to_string()
    }
}

#[async_trait]
impl ContainerService for DockerServiceImpl {
    async fn list_containers(&self, filter: ContainerFilter) -> Result<Vec<ContainerInfo>> {
        let containers: Vec<ContainerInfo> = self
            .client
            .list_containers(Some(list_options(&filter)))
            .await?
            .into_iter()
            .map(|container| container.into())
            .collect();

        Ok(narrow_to_requested_states(containers, &filter.states))
    }
}

/// The request that asks the daemon for `filter`, as far as it can express it.
fn list_options(filter: &ContainerFilter) -> ListContainersOptions {
    // `all` defaults to false, which hides everything that is not running.
    // Ask for all of them and let the status filter do the narrowing.
    let mut options = ListContainersOptionsBuilder::new()
        .all(true)
        .size(filter.with_size);

    let statuses: Vec<&str> = filter
        .states
        .iter()
        .filter_map(status_filter_value)
        .collect();
    if !statuses.is_empty() {
        options = options.filters(&HashMap::from([("status", statuses)]));
    }

    options.build()
}

/// The daemon's answer without whatever was not asked for.
///
/// Docker has no filter value for every state we model, so narrow what it
/// could not rather than hand back more than was asked for. An empty `states`
/// means every state and keeps the answer as it came.
fn narrow_to_requested_states(
    containers: Vec<ContainerInfo>,
    states: &[StateEnum],
) -> Vec<ContainerInfo> {
    if states.is_empty() {
        return containers;
    }

    containers
        .into_iter()
        .filter(|c| c.state.as_ref().is_some_and(|s| states.contains(s)))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    /// What `list_options` asks the daemon for.
    mod requesting {
        use super::*;

        /// The `status` values the built request carries, sorted for comparison.
        ///
        /// The filter is built from a map, so the order the daemon sees is not
        /// ours to predict and not ours to assert on.
        fn statuses(options: &ListContainersOptions) -> Option<Vec<String>> {
            let mut statuses = options.filters.as_ref()?.get("status")?.clone();
            statuses.sort();
            Some(statuses)
        }

        #[test]
        fn every_container_is_asked_for_whatever_the_filter_says() {
            // The daemon lists only running containers unless told otherwise, so
            // asking for stopped ones has to survive here or the status filter
            // below would be narrowing an already-narrowed list.
            for filter in [
                ContainerFilter::default(),
                ContainerFilter::running(),
                ContainerFilter::states([StateEnum::EXITED]),
            ] {
                assert!(list_options(&filter).all, "{filter:?}");
            }
        }

        #[test]
        fn sizes_are_only_computed_when_they_were_asked_for() {
            // Reporting sizes walks each container's filesystem, so a list view
            // that does not show them must not pay for them.
            assert!(!list_options(&ContainerFilter::default()).size);
            assert!(list_options(&ContainerFilter::default().with_size(true)).size);
        }

        #[test]
        fn an_unfiltered_request_carries_no_filters_at_all() {
            // Not an empty filter map: the daemon should do no narrowing.
            assert!(list_options(&ContainerFilter::default()).filters.is_none());
        }

        #[test]
        fn the_requested_states_travel_as_dockers_own_status_filter() {
            let options = list_options(&ContainerFilter::states([
                StateEnum::RUNNING,
                StateEnum::EXITED,
            ]));

            assert_eq!(
                statuses(&options),
                Some(vec!["exited".to_owned(), "running".to_owned()])
            );
        }

        #[test]
        fn states_docker_cannot_filter_on_are_left_out_of_the_request() {
            // A made-up filter value would make the daemon reject the request, so
            // `STOPPING` has to be dropped here and narrowed afterwards instead.
            let options = list_options(&ContainerFilter::states([
                StateEnum::RUNNING,
                StateEnum::STOPPING,
            ]));

            assert_eq!(statuses(&options), Some(vec!["running".to_owned()]));
        }

        #[test]
        fn a_filter_docker_understands_none_of_sends_no_status_filter() {
            // Sending `{"status": []}` is not the same request as sending none,
            // and it is not the one we want.
            let options = list_options(&ContainerFilter::states([
                StateEnum::STOPPING,
                StateEnum::EMPTY,
            ]));

            assert!(options.filters.is_none());
        }
    }

    /// What `narrow_to_requested_states` keeps of the answer.
    mod narrowing {
        use super::*;

        fn with_state(state: StateEnum) -> ContainerInfo {
            ContainerInfo {
                state: Some(state),
                ..Default::default()
            }
        }

        fn states_of(containers: &[ContainerInfo]) -> Vec<Option<StateEnum>> {
            containers.iter().map(|c| c.state.clone()).collect()
        }

        #[test]
        fn only_the_states_that_were_asked_for_are_kept() {
            let containers = vec![
                with_state(StateEnum::RUNNING),
                with_state(StateEnum::EXITED),
                with_state(StateEnum::PAUSED),
            ];

            let narrowed =
                narrow_to_requested_states(containers, &[StateEnum::RUNNING, StateEnum::PAUSED]);

            assert_eq!(
                states_of(&narrowed),
                vec![Some(StateEnum::RUNNING), Some(StateEnum::PAUSED)]
            );
        }

        #[test]
        fn this_is_what_enforces_a_state_docker_could_not_filter_on() {
            // The request for `STOPPING` carried no status filter, so the daemon
            // answered with everything. This is the only thing standing between
            // that answer and the caller.
            let containers = vec![
                with_state(StateEnum::RUNNING),
                with_state(StateEnum::STOPPING),
            ];

            let narrowed = narrow_to_requested_states(containers, &[StateEnum::STOPPING]);

            assert_eq!(states_of(&narrowed), vec![Some(StateEnum::STOPPING)]);
        }

        #[test]
        fn an_empty_filter_narrows_nothing() {
            // Empty means every state, not no state.
            let containers = vec![
                with_state(StateEnum::RUNNING),
                with_state(StateEnum::DEAD),
                ContainerInfo::default(),
            ];

            let narrowed = narrow_to_requested_states(containers, &[]);

            assert_eq!(
                states_of(&narrowed),
                vec![Some(StateEnum::RUNNING), Some(StateEnum::DEAD), None]
            );
        }

        #[test]
        fn a_container_of_unknown_state_is_dropped_once_states_were_asked_for() {
            // The daemon left the state out, so we cannot say it is one of the
            // requested ones — and the caller asked for those and nothing else.
            let containers = vec![ContainerInfo::default(), with_state(StateEnum::RUNNING)];

            let narrowed = narrow_to_requested_states(containers, &[StateEnum::RUNNING]);

            assert_eq!(states_of(&narrowed), vec![Some(StateEnum::RUNNING)]);
        }
    }
}
