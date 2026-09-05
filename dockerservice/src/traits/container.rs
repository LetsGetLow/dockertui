use crate::Result;
use crate::models::ContainerInfo;
use async_trait::async_trait;

#[async_trait]
pub trait ContainerService {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>>;
}
