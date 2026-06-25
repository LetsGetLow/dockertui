#[derive(Debug)]
pub struct ContainerInfo {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub image_id: String,
    pub command: String,
    pub created: Option<i64>,
    pub ports: Vec<Port>,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
    // pub labels: Option<HashMap<String, String>>,
    // pub state: Option<ContainerSummaryStateEnum>,
    pub status: String,
    // pub host_config: Option<ContainerSummaryHostConfig>,
    // pub network_settings: Option<ContainerSummaryNetworkSettings>,
    // pub mounts: Option<Vec<MountPoint>>,
    // pub health: Option<ContainerSummaryHealth>
}

#[derive(Debug)]
pub struct Port {
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    pub typ: Option<PortTypeEnum>,
}

#[derive(Debug)]
pub enum PortTypeEnum {
    EMPTY,
    TCP,
    UDP,
    SCTP
}

pub enum ContainerInfoStateEnum {
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