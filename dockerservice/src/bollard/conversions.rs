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
