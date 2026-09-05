use crate::types::{AnnotationsType, LabelsType, MountPointType};
use std::collections::HashMap;

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
    pub labels: Option<LabelsType>,
    pub state: Option<StateEnum>,
    pub status: Option<String>,
    pub host_config: Option<HostConfig>,
    pub network_settings: Option<NetworkSettings>,
    pub mounts: Option<Vec<MountPoint>>,
    pub health: Option<ContainerSummaryHealth>,
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
    SCTP,
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
    pub annotations: Option<AnnotationsType>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NetworkSettings {
    pub networks: Option<HashMap<String, EndpointSettings>>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EndpointSettings {
    pub ipam_config: Option<EndpointIpamConfig>,
    pub links: Option<Vec<String>>,
    pub mac_address: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub driver_opts: Option<HashMap<String, String>>,
    pub gw_priority: Option<i64>,
    pub network_id: Option<String>,
    pub endpoint_id: Option<String>,
    pub gateway: Option<String>,
    pub ip_address: Option<String>,
    pub ip_prefix_len: Option<i64>,
    pub ipv6_gateway: Option<String>,
    pub global_ipv6_address: Option<String>,
    pub global_ipv6_prefix_len: Option<i64>,
    pub dns_names: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EndpointIpamConfig {
    pub ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
    pub link_local_ips: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ContainerSummaryHealth {
    pub status: Option<HealthStatusEnum>,
    pub failing_streak: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatusEnum {
    EMPTY,
    NONE,
    STARTING,
    HEALTHY,
    UNHEALTHY,
}

/// Which containers to list, and how much work the daemon should do to answer.
///
/// The default asks for every container without sizes, which is what a list
/// view wants.
#[derive(Debug, Clone, Default)]
pub struct ContainerFilter {
    /// States to include. Empty means every state.
    pub states: Vec<StateEnum>,
    /// Ask the daemon to report `size_rw` and `size_root_fs`. This walks each
    /// container's filesystem, so it is off unless asked for.
    pub with_size: bool,
}

impl ContainerFilter {
    /// Only containers that are up.
    pub fn running() -> Self {
        Self::states([StateEnum::RUNNING])
    }

    pub fn states(states: impl IntoIterator<Item = StateEnum>) -> Self {
        Self {
            states: states.into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn with_size(mut self, with_size: bool) -> Self {
        self.with_size = with_size;
        self
    }
}
