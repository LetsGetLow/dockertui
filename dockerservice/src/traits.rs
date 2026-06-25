use async_trait::async_trait;
use crate::models::ContainerInfo;
use crate::Result;

#[async_trait]
pub trait DockerService {
    fn version(&self) -> String;
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>>;
}