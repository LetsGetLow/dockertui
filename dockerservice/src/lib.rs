pub(crate) mod traits;
pub(crate) mod types;

pub mod models;

pub use traits::{ContainerService, DockerService, SystemService};
pub use types::Result;

/// Brings every service trait into scope at once.
///
/// Method calls resolve only through the trait that declares them, so a
/// consumer touching several resources would otherwise import several traits.
pub mod prelude {
    pub use crate::traits::{ContainerService, DockerService, SystemService};
}

#[cfg(feature = "bollard-service")]
pub mod bollard;

#[cfg(feature = "bollard-service")]
pub use bollard::DockerServiceImpl;
