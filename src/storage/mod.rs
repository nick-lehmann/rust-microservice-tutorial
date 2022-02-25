use crate::model::{
    task::{Task, TaskID, TaskInput},
    user::UserID,
};

pub mod local;
pub mod postgres;

pub type StorageError = &'static str;
pub type StorageResult<T> = Result<T, StorageError>;

#[tonic::async_trait]
pub trait TasksStorage: Send + Sync + 'static {
    async fn list(&self) -> Vec<Task>;
    async fn create(&self, task_input: TaskInput) -> StorageResult<Task>;
    async fn list_tasks_done(&self, user_id: &UserID) -> Option<Vec<TaskID>>;
    async fn accomplish_task(&self, user_id: &UserID, task: &TaskID) -> ();
}
