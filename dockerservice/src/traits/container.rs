use crate::Result;
use crate::models::{ContainerFilter, ContainerInfo};
use async_trait::async_trait;

#[async_trait]
pub trait ContainerService: Send + Sync {
    async fn list_containers(&self, filter: ContainerFilter) -> Result<Vec<ContainerInfo>>;
}
