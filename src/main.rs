mod api;
mod model;
mod services;

use tonic::transport::Server;

use crate::{
    api::tasks::{TasksAPI, TasksServiceServer},
    services::tasks::LocalTasksService,
};

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    let tasks_service = LocalTasksService::default();

    let tasks_api = TasksAPI::new(tasks_service);

    println!("Starting up server on {}", addr);

    Server::builder()
        .add_service(TasksServiceServer::new(tasks_api))
        .serve(addr)
        .await?;

    Ok(())
}
