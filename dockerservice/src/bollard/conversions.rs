use crate::models::{ContainerInfo, Port, PortTypeEnum};
use bollard::models::{ContainerSummary, PortSummary, PortSummaryTypeEnum};

impl From<ContainerSummary> for ContainerInfo {
    fn from(summary: ContainerSummary) -> Self {
        Self {
            id: summary.id.unwrap_or_default(),
            names: summary.names.unwrap_or_default(),
            image: summary.image.unwrap_or_default(),
            image_id: summary.image_id.unwrap_or_default(),
            command: summary.command.unwrap_or_default(),
            created: summary.created,
            ports: summary
                .ports
                .unwrap_or_default()
                .into_iter()
                .map(|p| p.into())
                .collect(),
            size_rw: summary.size_rw,
            size_root_fs: summary.size_root_fs,
            status: summary.status.unwrap_or_default(),
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
