use crate::types::{AnnotationsType, LablesType, MountPointType};

#[derive(Debug, Clone, Default)]
pub struct ContainerInfo {
    pub id: Option<String>,
    pub names: Option<Vec<String>>,
    pub image: Option<String>,
    pub image_id: Option<String>,
    pub command: Option<String>,
    pub created: Option<i64>,
    pub ports: Option<Vec<Port>>,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
    pub labels: Option<LablesType>,
    pub state: Option<StateEnum>,
    pub status: Option<String>,
    pub host_config: Option<HostConfig>,
    // pub network_settings: Option<ContainerSummaryNetworkSettings>,
    pub mounts: Option<Vec<MountPoint>>,
    // pub health: Option<ContainerSummaryHealth>
}

#[derive(Debug, Clone, Default)]
pub struct Port {
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    pub typ: Option<PortTypeEnum>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PortTypeEnum {
    EMPTY,
    TCP,
    UDP,
    SCTP
}


#[derive(Debug, Clone, PartialEq)]
pub enum StateEnum {
    EMPTY,
    CREATED,
    RUNNING,
    PAUSED,
    RESTARTING,
    EXITED,
    REMOVING,
    DEAD,
    STOPPING,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MountPoint {
    pub typ: Option<MountPointType>,
    pub name: Option<String>,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub driver: Option<String>,
    pub mode: Option<String>,
    pub rw: Option<bool>,
    pub propagation: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HostConfig {
    pub network_mode: Option<String>,
    pub annotations: Option<AnnotationsType>
}