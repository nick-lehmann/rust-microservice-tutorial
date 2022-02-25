use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::RwLock;

use crate::model::task as model;

use crate::model::{
    task::{Task, TaskID},
    user::UserID,
};

use super::{StorageResult, TasksStorage};

type LogsDB = Arc<Mutex<HashMap<String, Vec<TaskID>>>>;

pub struct LocalTasksStorage {
    tasks: RwLock<Vec<Task>>,
    logs: LogsDB,
}

#[tonic::async_trait]
impl TasksStorage for LocalTasksStorage {
    async fn list(&self) -> Vec<Task> {
        self.tasks.read().await.clone()
    }

    async fn create(&self, task_input: model::TaskInput) -> StorageResult<model::Task> {
        let task = Task {
            id: 1,
            name: task_input.name,
            description: task_input.description,
        };

        self.tasks.write().await.push(task.clone());
        Ok(task)
    }

    async fn list_tasks_done(&self, user_id: &UserID) -> Option<Vec<model::TaskID>> {
        let logs = self.logs.lock().unwrap();
        logs.get(user_id).cloned()
    }

    async fn accomplish_task(&self, user_id: &UserID, task: &model::TaskID) -> () {
        let mut logs = self.logs.lock().unwrap();
        let mut task_list = match logs.get(&user_id.to_string()) {
            Some(task_list) => task_list.clone(),
            None => Vec::new(),
        };
        task_list.push(task.to_owned());
        logs.insert(user_id.to_string(), task_list);
    }
}
