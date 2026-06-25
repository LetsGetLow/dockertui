use dockerservice::{DockerServiceImpl, DockerService};

#[tokio::main]
async fn main() {
    let service = DockerServiceImpl::new().unwrap();
    println!("Service Version: {}", service.version());

    for container in service.list_containers().await.unwrap() {
        println!("ID: {}", container.id);
        println!("Names: {:?}", container.names);
        println!("Image: {}", container.image);
        println!("ImageID: {}", container.image_id);
        println!("Command: {}", container.command);
        println!("Created: {:?}", container.created);
        println!("Ports: {:?}", container.ports);
        println!("SizeRW: {:?}", container.size_rw);
        println!("SizeRootFS: {:?}", container.size_root_fs);
        println!("Status: {:?}", container.status);
        println!("---------------------------------------------------------");
    }
}
