#[macro_use]
extern crate diesel;

mod api;
mod model;
mod services;
mod storage;

use std::net::SocketAddr;
use tonic::transport::Server;

use crate::{
    api::tasks::{TasksAPI, TasksServiceServer},
    services::tasks::SimpleTasksService,
    storage::postgres::service::{get_pool, PostgresStorage},
};

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;

    let tasks_storage = PostgresStorage::new(get_pool());
    let tasks_service = SimpleTasksService::new(tasks_storage);

    let tasks_api = TasksAPI::new(tasks_service);

    println!("Starting up server on {}", addr);

    Server::builder()
        .add_service(TasksServiceServer::new(tasks_api))
        .serve(addr)
        .await?;

    Ok(())
}
