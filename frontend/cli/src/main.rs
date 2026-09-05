use dockerservice::DockerServiceImpl;
use dockerservice::prelude::*;

#[tokio::main]
async fn main() {
    let service = DockerServiceImpl::new().unwrap();
    println!("Service Version: {}", service.version());

    for container in service.list_containers().await.unwrap() {
        println!("ID: {}", container.id.unwrap_or_default());
        println!("Names: {:?}", container.names);
        println!("Image: {}", container.image.unwrap_or_default());
        println!("ImageID: {}", container.image_id.unwrap_or_default());
        println!("Command: {}", container.command.unwrap_or_default());
        println!("Created: {:?}", container.created);
        println!("Ports: {:?}", container.ports);
        println!("SizeRW: {:?}", container.size_rw);
        println!("SizeRootFS: {:?}", container.size_root_fs);
        println!("Labels: {:?}", container.labels);
        println!("State: {:?}", container.state);
        println!("Status: {:?}", container.status);
        println!("Host-Config: {:?}", container.host_config);
        println!("Network-Settings: {:?}", container.network_settings);
        println!("Mounts: {:?}", container.mounts);
        println!("---------------------------------------------------------");
    }
}
