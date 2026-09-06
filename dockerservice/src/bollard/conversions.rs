use crate::models::{
    ContainerInfo, ContainerSummaryHealth, EndpointIpamConfig, EndpointSettings, HealthStatusEnum,
    HostConfig, MountPoint, NetworkSettings, Port, PortTypeEnum, StateEnum,
};
use bollard::models::{
    ContainerSummary, ContainerSummaryHealth as BollardContainerSummaryHealth,
    ContainerSummaryHealthStatusEnum, ContainerSummaryHostConfig, ContainerSummaryNetworkSettings,
    ContainerSummaryStateEnum, EndpointIpamConfig as BollardEndpointIpamConfig,
    EndpointSettings as BollardEndpointSettings, MountPoint as BollardMountPoint, PortSummary,
    PortSummaryTypeEnum,
};

impl From<ContainerSummary> for ContainerInfo {
    fn from(summary: ContainerSummary) -> Self {
        Self {
            id: summary.id,
            names: summary.names,
            image: summary.image,
            image_id: summary.image_id,
            command: summary.command,
            created: summary.created,
            ports: summary
                .ports
                .map(|p| p.into_iter().map(Into::into).collect()),
            size_rw: summary.size_rw,
            size_root_fs: summary.size_root_fs,
            labels: summary.labels,
            state: summary.state.map(|s| s.into()),
            status: summary.status,
            host_config: summary.host_config.map(|hc| hc.into()),
            network_settings: summary.network_settings.map(|ns| ns.into()),
            mounts: summary
                .mounts
                .map(|m| m.into_iter().map(|m| m.into()).collect()),
            health: summary.health.map(|h| h.into()),
        }
    }
}

impl From<PortSummary> for Port {
    fn from(port: PortSummary) -> Self {
        Self {
            ip: port.ip,
            private_port: port.private_port,
            public_port: port.public_port,
            typ: port.typ.map(|t| t.into()),
        }
    }
}

impl From<PortSummaryTypeEnum> for PortTypeEnum {
    fn from(typ: PortSummaryTypeEnum) -> Self {
        match typ {
            PortSummaryTypeEnum::EMPTY => PortTypeEnum::EMPTY,
            PortSummaryTypeEnum::TCP => PortTypeEnum::TCP,
            PortSummaryTypeEnum::UDP => PortTypeEnum::UDP,
            PortSummaryTypeEnum::SCTP => PortTypeEnum::SCTP,
        }
    }
}

impl From<ContainerSummaryStateEnum> for StateEnum {
    fn from(state: ContainerSummaryStateEnum) -> Self {
        match state {
            ContainerSummaryStateEnum::EMPTY => StateEnum::EMPTY,
            ContainerSummaryStateEnum::CREATED => StateEnum::CREATED,
            ContainerSummaryStateEnum::RUNNING => StateEnum::RUNNING,
            ContainerSummaryStateEnum::PAUSED => StateEnum::PAUSED,
            ContainerSummaryStateEnum::RESTARTING => StateEnum::RESTARTING,
            ContainerSummaryStateEnum::EXITED => StateEnum::EXITED,
            ContainerSummaryStateEnum::REMOVING => StateEnum::REMOVING,
            ContainerSummaryStateEnum::DEAD => StateEnum::DEAD,
            ContainerSummaryStateEnum::STOPPING => StateEnum::STOPPING,
        }
    }
}

impl From<BollardMountPoint> for MountPoint {
    fn from(mount: BollardMountPoint) -> Self {
        Self {
            typ: mount.typ,
            name: mount.name,
            source: mount.source,
            destination: mount.destination,
            driver: mount.driver,
            mode: mount.mode,
            rw: mount.rw,
            propagation: mount.propagation,
        }
    }
}

impl From<ContainerSummaryHostConfig> for HostConfig {
    fn from(config: ContainerSummaryHostConfig) -> Self {
        Self {
            network_mode: config.network_mode,
            annotations: config.annotations,
        }
    }
}

impl From<ContainerSummaryNetworkSettings> for NetworkSettings {
    fn from(settings: ContainerSummaryNetworkSettings) -> Self {
        Self {
            networks: settings.networks.map(|networks| {
                networks
                    .into_iter()
                    .map(|(name, network)| (name, network.into()))
                    .collect()
            }),
        }
    }
}

impl From<BollardEndpointSettings> for EndpointSettings {
    fn from(settings: BollardEndpointSettings) -> Self {
        Self {
            ipam_config: settings.ipam_config.map(Into::into),
            links: settings.links,
            mac_address: settings.mac_address,
            aliases: settings.aliases,
            driver_opts: settings.driver_opts,
            gw_priority: settings.gw_priority,
            network_id: settings.network_id,
            endpoint_id: settings.endpoint_id,
            gateway: settings.gateway,
            ip_address: settings.ip_address,
            ip_prefix_len: settings.ip_prefix_len,
            ipv6_gateway: settings.ipv6_gateway,
            global_ipv6_address: settings.global_ipv6_address,
            global_ipv6_prefix_len: settings.global_ipv6_prefix_len,
            dns_names: settings.dns_names,
        }
    }
}

impl From<BollardEndpointIpamConfig> for EndpointIpamConfig {
    fn from(config: BollardEndpointIpamConfig) -> Self {
        Self {
            ipv4_address: config.ipv4_address,
            ipv6_address: config.ipv6_address,
            link_local_ips: config.link_local_ips,
        }
    }
}

impl From<BollardContainerSummaryHealth> for ContainerSummaryHealth {
    fn from(health: BollardContainerSummaryHealth) -> Self {
        Self {
            status: health.status.map(|s| s.into()),
            failing_streak: health.failing_streak,
        }
    }
}

impl From<ContainerSummaryHealthStatusEnum> for HealthStatusEnum {
    fn from(status: ContainerSummaryHealthStatusEnum) -> Self {
        match status {
            ContainerSummaryHealthStatusEnum::EMPTY => HealthStatusEnum::EMPTY,
            ContainerSummaryHealthStatusEnum::NONE => HealthStatusEnum::NONE,
            ContainerSummaryHealthStatusEnum::STARTING => HealthStatusEnum::STARTING,
            ContainerSummaryHealthStatusEnum::HEALTHY => HealthStatusEnum::HEALTHY,
            ContainerSummaryHealthStatusEnum::UNHEALTHY => HealthStatusEnum::UNHEALTHY,
        }
    }
}

/// The value Docker's `status` filter uses for a state, where it has one.
///
/// Docker documents no filter value for every state we model, so this is
/// `None` for the rest and the caller narrows those itself.
pub(super) fn status_filter_value(state: &StateEnum) -> Option<&'static str> {
    match state {
        StateEnum::CREATED => Some("created"),
        StateEnum::RESTARTING => Some("restarting"),
        StateEnum::RUNNING => Some("running"),
        StateEnum::REMOVING => Some("removing"),
        StateEnum::PAUSED => Some("paused"),
        StateEnum::EXITED => Some("exited"),
        StateEnum::DEAD => Some("dead"),
        StateEnum::EMPTY | StateEnum::STOPPING => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// A summary with a distinct value in every field.
    ///
    /// Distinct values are the point: fields of the same type sitting next to
    /// each other (`image`/`image_id`, `size_rw`/`size_root_fs`) would let a
    /// transposition pass unnoticed if they shared a value.
    fn full_summary() -> ContainerSummary {
        ContainerSummary {
            id: Some("abc123".to_owned()),
            names: Some(vec!["/web".to_owned(), "/web-alias".to_owned()]),
            image: Some("nginx:latest".to_owned()),
            image_id: Some("sha256:deadbeef".to_owned()),
            command: Some("nginx -g daemon off;".to_owned()),
            created: Some(1_700_000_000),
            ports: Some(vec![PortSummary {
                ip: Some("0.0.0.0".to_owned()),
                private_port: 80,
                public_port: Some(8080),
                typ: Some(PortSummaryTypeEnum::TCP),
            }]),
            size_rw: Some(1024),
            size_root_fs: Some(2048),
            labels: Some(HashMap::from([("app".to_owned(), "web".to_owned())])),
            state: Some(ContainerSummaryStateEnum::RUNNING),
            status: Some("Up 2 hours".to_owned()),
            host_config: Some(ContainerSummaryHostConfig {
                network_mode: Some("bridge".to_owned()),
                annotations: Some(HashMap::from([("k".to_owned(), "v".to_owned())])),
            }),
            network_settings: Some(ContainerSummaryNetworkSettings {
                networks: Some(HashMap::from([(
                    "bridge".to_owned(),
                    full_endpoint_settings(),
                )])),
            }),
            mounts: Some(vec![full_mount_point()]),
            health: Some(BollardContainerSummaryHealth {
                status: Some(ContainerSummaryHealthStatusEnum::HEALTHY),
                failing_streak: Some(0),
            }),
            ..Default::default()
        }
    }

    fn full_mount_point() -> BollardMountPoint {
        BollardMountPoint {
            typ: Some("volume".to_owned()),
            name: Some("data".to_owned()),
            source: Some("/var/lib/docker/volumes/data/_data".to_owned()),
            destination: Some("/data".to_owned()),
            driver: Some("local".to_owned()),
            mode: Some("z".to_owned()),
            rw: Some(true),
            propagation: Some("rprivate".to_owned()),
        }
    }

    fn full_endpoint_settings() -> BollardEndpointSettings {
        BollardEndpointSettings {
            ipam_config: Some(BollardEndpointIpamConfig {
                ipv4_address: Some("172.17.0.2".to_owned()),
                ipv6_address: Some("2001:db8::2".to_owned()),
                link_local_ips: Some(vec!["169.254.0.1".to_owned()]),
            }),
            links: Some(vec!["db:db".to_owned()]),
            mac_address: Some("02:42:ac:11:00:02".to_owned()),
            aliases: Some(vec!["web".to_owned()]),
            driver_opts: Some(HashMap::from([("opt".to_owned(), "val".to_owned())])),
            gw_priority: Some(7),
            network_id: Some("net-id".to_owned()),
            endpoint_id: Some("endpoint-id".to_owned()),
            gateway: Some("172.17.0.1".to_owned()),
            ip_address: Some("172.17.0.2".to_owned()),
            ip_prefix_len: Some(16),
            ipv6_gateway: Some("2001:db8::1".to_owned()),
            global_ipv6_address: Some("2001:db8::2".to_owned()),
            global_ipv6_prefix_len: Some(64),
            dns_names: Some(vec!["web.local".to_owned()]),
        }
    }

    #[test]
    fn every_summary_field_lands_in_its_counterpart() {
        let info: ContainerInfo = full_summary().into();

        assert_eq!(info.id.as_deref(), Some("abc123"));
        assert_eq!(
            info.names,
            Some(vec!["/web".to_owned(), "/web-alias".to_owned()])
        );
        assert_eq!(info.image.as_deref(), Some("nginx:latest"));
        assert_eq!(info.image_id.as_deref(), Some("sha256:deadbeef"));
        assert_eq!(info.command.as_deref(), Some("nginx -g daemon off;"));
        assert_eq!(info.created, Some(1_700_000_000));
        assert_eq!(info.size_rw, Some(1024));
        assert_eq!(info.size_root_fs, Some(2048));
        assert_eq!(
            info.labels,
            Some(HashMap::from([("app".to_owned(), "web".to_owned())]))
        );
        assert_eq!(info.state, Some(StateEnum::RUNNING));
        assert_eq!(info.status.as_deref(), Some("Up 2 hours"));
        assert_eq!(
            info.host_config,
            Some(HostConfig {
                network_mode: Some("bridge".to_owned()),
                annotations: Some(HashMap::from([("k".to_owned(), "v".to_owned())])),
            })
        );
        assert_eq!(
            info.health,
            Some(ContainerSummaryHealth {
                status: Some(HealthStatusEnum::HEALTHY),
                failing_streak: Some(0),
            })
        );
    }

    #[test]
    fn nothing_is_invented_for_a_summary_that_carries_nothing() {
        // The daemon omits most of these unless asked, so a container the UI
        // knows nothing about must not come out looking populated.
        let info: ContainerInfo = ContainerSummary::default().into();

        assert!(info.id.is_none());
        assert!(info.names.is_none());
        assert!(info.image.is_none());
        assert!(info.image_id.is_none());
        assert!(info.command.is_none());
        assert!(info.created.is_none());
        assert!(info.ports.is_none());
        assert!(info.size_rw.is_none());
        assert!(info.size_root_fs.is_none());
        assert!(info.labels.is_none());
        assert!(info.state.is_none());
        assert!(info.status.is_none());
        assert!(info.host_config.is_none());
        assert!(info.network_settings.is_none());
        assert!(info.mounts.is_none());
        assert!(info.health.is_none());
    }

    #[test]
    fn the_nested_collections_are_converted_element_by_element() {
        let info: ContainerInfo = full_summary().into();

        let ports = info.ports.expect("ports");
        assert_eq!(ports.len(), 1);
        assert_eq!(ports[0].private_port, 80);

        let mounts = info.mounts.expect("mounts");
        assert_eq!(mounts, vec![MountPoint::from(full_mount_point())]);

        let networks = info
            .network_settings
            .expect("network settings")
            .networks
            .expect("networks");
        assert_eq!(
            networks,
            HashMap::from([(
                "bridge".to_owned(),
                EndpointSettings::from(full_endpoint_settings())
            )])
        );
    }

    #[test]
    fn empty_collections_survive_as_empty_rather_than_absent() {
        // `Some(vec![])` and `None` mean different things to a caller: no
        // published ports versus a daemon that did not say.
        let summary = ContainerSummary {
            ports: Some(vec![]),
            mounts: Some(vec![]),
            ..Default::default()
        };
        let info: ContainerInfo = summary.into();

        assert_eq!(info.ports.map(|p| p.len()), Some(0));
        assert_eq!(info.mounts.map(|m| m.len()), Some(0));
    }

    #[test]
    fn a_port_keeps_its_host_and_container_side_apart() {
        let port: Port = PortSummary {
            ip: Some("127.0.0.1".to_owned()),
            private_port: 5432,
            public_port: Some(15432),
            typ: Some(PortSummaryTypeEnum::TCP),
        }
        .into();

        assert_eq!(port.ip.as_deref(), Some("127.0.0.1"));
        assert_eq!(port.private_port, 5432);
        assert_eq!(port.public_port, Some(15432));
        assert_eq!(port.typ, Some(PortTypeEnum::TCP));
    }

    #[test]
    fn an_unpublished_port_has_no_host_side() {
        let port: Port = PortSummary {
            private_port: 5432,
            ..Default::default()
        }
        .into();

        assert!(port.ip.is_none());
        assert!(port.public_port.is_none());
        assert!(port.typ.is_none());
    }

    #[test]
    fn every_port_type_maps_to_its_own_variant() {
        let pairs = [
            (PortSummaryTypeEnum::EMPTY, PortTypeEnum::EMPTY),
            (PortSummaryTypeEnum::TCP, PortTypeEnum::TCP),
            (PortSummaryTypeEnum::UDP, PortTypeEnum::UDP),
            (PortSummaryTypeEnum::SCTP, PortTypeEnum::SCTP),
        ];

        for (from, expected) in pairs {
            assert_eq!(PortTypeEnum::from(from), expected, "{from:?}");
        }
    }

    #[test]
    fn every_state_maps_to_its_own_variant() {
        let pairs = [
            (ContainerSummaryStateEnum::EMPTY, StateEnum::EMPTY),
            (ContainerSummaryStateEnum::CREATED, StateEnum::CREATED),
            (ContainerSummaryStateEnum::RUNNING, StateEnum::RUNNING),
            (ContainerSummaryStateEnum::PAUSED, StateEnum::PAUSED),
            (ContainerSummaryStateEnum::RESTARTING, StateEnum::RESTARTING),
            (ContainerSummaryStateEnum::EXITED, StateEnum::EXITED),
            (ContainerSummaryStateEnum::REMOVING, StateEnum::REMOVING),
            (ContainerSummaryStateEnum::DEAD, StateEnum::DEAD),
            (ContainerSummaryStateEnum::STOPPING, StateEnum::STOPPING),
        ];

        for (from, expected) in pairs {
            assert_eq!(StateEnum::from(from), expected, "{from:?}");
        }
    }

    #[test]
    fn every_health_status_maps_to_its_own_variant() {
        let pairs = [
            (
                ContainerSummaryHealthStatusEnum::EMPTY,
                HealthStatusEnum::EMPTY,
            ),
            (
                ContainerSummaryHealthStatusEnum::NONE,
                HealthStatusEnum::NONE,
            ),
            (
                ContainerSummaryHealthStatusEnum::STARTING,
                HealthStatusEnum::STARTING,
            ),
            (
                ContainerSummaryHealthStatusEnum::HEALTHY,
                HealthStatusEnum::HEALTHY,
            ),
            (
                ContainerSummaryHealthStatusEnum::UNHEALTHY,
                HealthStatusEnum::UNHEALTHY,
            ),
        ];

        for (from, expected) in pairs {
            assert_eq!(HealthStatusEnum::from(from), expected, "{from:?}");
        }
    }

    #[test]
    fn a_mount_point_keeps_source_and_destination_apart() {
        let mount: MountPoint = full_mount_point().into();

        assert_eq!(mount.typ.as_deref(), Some("volume"));
        assert_eq!(mount.name.as_deref(), Some("data"));
        assert_eq!(
            mount.source.as_deref(),
            Some("/var/lib/docker/volumes/data/_data")
        );
        assert_eq!(mount.destination.as_deref(), Some("/data"));
        assert_eq!(mount.driver.as_deref(), Some("local"));
        assert_eq!(mount.mode.as_deref(), Some("z"));
        assert_eq!(mount.rw, Some(true));
        assert_eq!(mount.propagation.as_deref(), Some("rprivate"));
    }

    #[test]
    fn a_read_only_mount_stays_read_only() {
        // `rw: Some(false)` must not collapse into `None` on the way through.
        let mount: MountPoint = BollardMountPoint {
            rw: Some(false),
            ..Default::default()
        }
        .into();

        assert_eq!(mount.rw, Some(false));
    }

    #[test]
    fn a_host_config_carries_its_network_mode_and_annotations() {
        let config: HostConfig = ContainerSummaryHostConfig {
            network_mode: Some("host".to_owned()),
            annotations: Some(HashMap::from([("a".to_owned(), "b".to_owned())])),
        }
        .into();

        assert_eq!(config.network_mode.as_deref(), Some("host"));
        assert_eq!(
            config.annotations,
            Some(HashMap::from([("a".to_owned(), "b".to_owned())]))
        );
    }

    #[test]
    fn networks_keep_the_name_they_are_keyed_by() {
        let settings: NetworkSettings = ContainerSummaryNetworkSettings {
            networks: Some(HashMap::from([
                ("bridge".to_owned(), BollardEndpointSettings::default()),
                (
                    "backend".to_owned(),
                    BollardEndpointSettings {
                        ip_address: Some("10.0.0.2".to_owned()),
                        ..Default::default()
                    },
                ),
            ])),
        }
        .into();

        let networks = settings.networks.expect("networks");
        assert_eq!(networks.len(), 2);
        assert_eq!(
            networks["backend"].ip_address.as_deref(),
            Some("10.0.0.2"),
            "the entry must stay attached to its own key"
        );
        assert!(networks["bridge"].ip_address.is_none());
    }

    #[test]
    fn a_container_off_the_network_has_no_networks() {
        let settings: NetworkSettings = ContainerSummaryNetworkSettings::default().into();

        assert!(settings.networks.is_none());
    }

    #[test]
    fn every_endpoint_field_lands_in_its_counterpart() {
        // The v4 and v6 addresses, gateways and prefix lengths are the fields
        // most likely to be crossed, so each carries a value of its own.
        let settings: EndpointSettings = full_endpoint_settings().into();

        assert_eq!(settings.links, Some(vec!["db:db".to_owned()]));
        assert_eq!(settings.mac_address.as_deref(), Some("02:42:ac:11:00:02"));
        assert_eq!(settings.aliases, Some(vec!["web".to_owned()]));
        assert_eq!(
            settings.driver_opts,
            Some(HashMap::from([("opt".to_owned(), "val".to_owned())]))
        );
        assert_eq!(settings.gw_priority, Some(7));
        assert_eq!(settings.network_id.as_deref(), Some("net-id"));
        assert_eq!(settings.endpoint_id.as_deref(), Some("endpoint-id"));
        assert_eq!(settings.gateway.as_deref(), Some("172.17.0.1"));
        assert_eq!(settings.ip_address.as_deref(), Some("172.17.0.2"));
        assert_eq!(settings.ip_prefix_len, Some(16));
        assert_eq!(settings.ipv6_gateway.as_deref(), Some("2001:db8::1"));
        assert_eq!(settings.global_ipv6_address.as_deref(), Some("2001:db8::2"));
        assert_eq!(settings.global_ipv6_prefix_len, Some(64));
        assert_eq!(settings.dns_names, Some(vec!["web.local".to_owned()]));

        let ipam = settings.ipam_config.expect("ipam config");
        assert_eq!(ipam.ipv4_address.as_deref(), Some("172.17.0.2"));
        assert_eq!(ipam.ipv6_address.as_deref(), Some("2001:db8::2"));
        assert_eq!(ipam.link_local_ips, Some(vec!["169.254.0.1".to_owned()]));
    }

    #[test]
    fn an_endpoint_without_ipam_config_has_none() {
        let settings: EndpointSettings = BollardEndpointSettings::default().into();

        assert!(settings.ipam_config.is_none());
    }

    #[test]
    fn health_carries_the_status_and_the_streak() {
        let health: ContainerSummaryHealth = BollardContainerSummaryHealth {
            status: Some(ContainerSummaryHealthStatusEnum::UNHEALTHY),
            failing_streak: Some(3),
        }
        .into();

        assert_eq!(health.status, Some(HealthStatusEnum::UNHEALTHY));
        assert_eq!(health.failing_streak, Some(3));
    }

    #[test]
    fn a_container_without_a_health_check_reports_no_status() {
        let health: ContainerSummaryHealth = BollardContainerSummaryHealth::default().into();

        assert!(health.status.is_none());
        assert!(health.failing_streak.is_none());
    }

    #[test]
    fn each_filterable_state_uses_dockers_own_spelling() {
        let pairs = [
            (StateEnum::CREATED, "created"),
            (StateEnum::RESTARTING, "restarting"),
            (StateEnum::RUNNING, "running"),
            (StateEnum::REMOVING, "removing"),
            (StateEnum::PAUSED, "paused"),
            (StateEnum::EXITED, "exited"),
            (StateEnum::DEAD, "dead"),
        ];

        for (state, expected) in pairs {
            assert_eq!(status_filter_value(&state), Some(expected), "{state:?}");
        }
    }

    #[test]
    fn states_docker_cannot_filter_on_yield_no_value() {
        // Handing the daemon a made-up filter value would make it reject the
        // whole request, so these have to come back as `None` for the caller
        // to narrow itself.
        assert_eq!(status_filter_value(&StateEnum::EMPTY), None);
        assert_eq!(status_filter_value(&StateEnum::STOPPING), None);
    }
}
