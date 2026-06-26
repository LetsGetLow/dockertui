use crate::models::{ContainerInfo, HostConfig, MountPoint, Port, PortTypeEnum, StateEnum};
use bollard::models::{ContainerSummary, ContainerSummaryHostConfig, ContainerSummaryStateEnum, MountPoint as BollardMountPoint, PortSummary, PortSummaryTypeEnum};

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
            mounts: summary.mounts.map(|m| m.into_iter().map(|m| m.into()).collect()),
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