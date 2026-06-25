use crate::types::{Lables, MountPointType};

#[derive(Debug)]
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
    pub labels: Option<Lables>,
    pub state: Option<StateEnum>,
    pub status: Option<String>,
    // pub host_config: Option<ContainerSummaryHostConfig>,
    // pub network_settings: Option<ContainerSummaryNetworkSettings>,
    pub mounts: Option<Vec<MountPoint>>,
    // pub health: Option<ContainerSummaryHealth>
}

#[derive(Debug)]
pub struct Port {
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    pub typ: Option<PortTypeEnum>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PortTypeEnum {
    EMPTY,
    TCP,
    UDP,
    SCTP
}


#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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
