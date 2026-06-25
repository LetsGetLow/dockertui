use crate::models::ContainerInfo;
use crate::DockerService;
use crate::Result;
use async_trait::async_trait;
use bollard::Docker;

pub struct DockerServiceImpl {
    client: Docker,
}

impl DockerServiceImpl {
    pub fn new() -> Result<Self> {
        #[cfg(target_family = "windows")]
        unimplemented!();

        #[cfg(target_family = "unix")]
        let client = Docker::connect_with_local_defaults()?;

        Ok(Self { client })
    }
}

#[async_trait]
impl DockerService for DockerServiceImpl {
    fn version(&self) -> String {
        self.client.client_version().to_string()
    }

    async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
        Ok(self
            .client
            .list_containers(None)
            .await?
            .into_iter()
            .map(|container| container.into())
            .collect())
    }
}
