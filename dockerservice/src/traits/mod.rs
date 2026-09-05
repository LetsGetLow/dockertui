mod container;
mod system;

pub use container::ContainerService;
pub use system::SystemService;

/// Everything a Docker backend can do.
///
/// This is a bundle, not a definition: it declares no methods of its own and is
/// implemented for free by anything that implements each part. Depend on it when
/// you want a single handle to the whole daemon; depend on one part when that is
/// all you need, so a test double only has to implement that part.
pub trait DockerService: ContainerService + SystemService {}

impl<T: ContainerService + SystemService> DockerService for T {}
