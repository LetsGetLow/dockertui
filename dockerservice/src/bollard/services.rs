use super::conversions::state_filter_value;
use crate::models::{ContainerFilter, ContainerInfo};
use crate::{ContainerService, SystemService};
use crate::Result;
use async_trait::async_trait;
use bollard::Docker;
use bollard::query_parameters::ListContainersOptionsBuilder;
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
        // `all` defaults to false, which hides everything that is not running.
        // Ask for all of them and let the status filter do the narrowing.
        let mut options = ListContainersOptionsBuilder::new()
            .all(true)
            .size(filter.with_size);

        let statuses: Vec<&str> = filter.states.iter().filter_map(state_filter_value).collect();
        if !statuses.is_empty() {
            options = options.filters(&HashMap::from([("status", statuses)]));
        }

        let mut containers: Vec<ContainerInfo> = self
            .client
            .list_containers(Some(options.build()))
            .await?
            .into_iter()
            .map(|container| container.into())
            .collect();

        // Docker has no filter value for every state we model, so narrow what
        // it could not rather than hand back more than was asked for.
        if !filter.states.is_empty() {
            containers.retain(|c| {
                c.state.as_ref().is_some_and(|s| filter.states.contains(s))
            });
        }

        Ok(containers)
    }
}
