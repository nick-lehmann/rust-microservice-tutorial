use tonic::{Code, Request, Response, Status};

use crate::{
    model::task::{Task, TaskInput},
    services::tasks::TasksService,
};

use self::proto::tasks_service_server::TasksService as TasksAPIService;
pub use self::proto::tasks_service_server::TasksServiceServer;

pub mod proto {
    tonic::include_proto!("tasks");
}

impl Into<Task> for proto::Task {
    fn into(self) -> Task {
        Task {
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }
}

impl From<Task> for proto::Task {
    fn from(task: Task) -> Self {
        proto::Task {
            id: task.id,
            name: task.name,
            description: task.description,
            moods: vec![],
        }
    }
}

impl TryInto<TaskInput> for proto::CreateTaskRequest {
    type Error = Status;

    fn try_into(self) -> Result<TaskInput, Self::Error> {
        let proto_task = match self.task {
            Some(task) => task,
            None => return Err(Status::new(Code::InvalidArgument, "Task is required")),
        };

        Ok(TaskInput {
            name: proto_task.name,
            description: proto_task.description,
        })
    }
}

pub struct TasksAPI<Service: TasksService> {
    service: Service,
}

impl<Service: TasksService> TasksAPI<Service> {
    pub fn new(service: Service) -> Self {
        TasksAPI { service }
    }
}

#[tonic::async_trait]
impl<Service: TasksService + Send + Sync + 'static> TasksAPIService for TasksAPI<Service> {
    async fn list_tasks(
        &self,
        _request: Request<proto::ListTaskRequest>,
    ) -> Result<Response<proto::ListTaskResponse>, Status> {
        let tasks: Vec<proto::Task> = self
            .service
            .list_tasks()
            .await
            .into_iter()
            .map(|task| task.into())
            .collect();
        Ok(Response::new(proto::ListTaskResponse {
            tasks: tasks,
            next_page_token: "".to_string(),
        }))
    }

    async fn create_task(
        &self,
        request: Request<proto::CreateTaskRequest>,
    ) -> Result<Response<proto::Task>, Status> {
        let new_task = self
            .service
            .create_task(request.into_inner().try_into()?)
            .await;

        Ok(Response::new(new_task.into()))
    }

    async fn accomplish_task(
        &self,
        request: Request<proto::AccomplishTaskRequest>,
    ) -> Result<Response<proto::AccomplishTaskResponse>, Status> {
        let body = request.into_inner();

        self.service
            .accomplish_task(&body.user_id, &body.task_id)
            .await;

        Ok(Response::new(proto::AccomplishTaskResponse {
            user_id: body.user_id,
            task_id: body.task_id,
        }))
    }

    async fn list_accomplished_tasks(
        &self,
        request: Request<proto::ListAccomplishedTasksRequest>,
    ) -> Result<Response<proto::ListAccomplishedTasksResponse>, Status> {
        let body = request.into_inner();

        let tasks = self
            .service
            .list_accomplished_tasks(&body.user_id)
            .await
            .unwrap_or_default();

        Ok(Response::new(proto::ListAccomplishedTasksResponse {
            task_ids: tasks,
        }))
    }
}
