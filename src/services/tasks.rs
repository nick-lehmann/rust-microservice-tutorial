use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::RwLock;

use crate::model::{
    task::{Task, TaskID},
    user::UserID,
};

type LogsDB = Arc<Mutex<HashMap<String, Vec<TaskID>>>>;

#[tonic::async_trait]
pub trait TasksService {
    async fn list_tasks(&self) -> Vec<Task>;
    async fn create_task(&self, task: Task) -> Task;
    async fn list_accomplished_tasks(&self, user_id: &UserID) -> Option<Vec<TaskID>>;
    async fn accomplish_task(&self, user_id: &UserID, task: &TaskID);
}

pub struct LocalTasksService {
    tasks: RwLock<Vec<Task>>,
    logs: LogsDB,
}

impl Default for LocalTasksService {
    fn default() -> Self {
        Self {
            tasks: Default::default(),
            logs: Default::default(),
        }
    }
}

impl LocalTasksService {}

#[tonic::async_trait]
impl TasksService for LocalTasksService {
    async fn list_tasks(&self) -> Vec<Task> {
        self.tasks.read().await.clone()
    }

    async fn create_task(&self, task: Task) -> Task {
        self.tasks.write().await.push(task.clone());
        task
    }

    async fn list_accomplished_tasks(&self, user_id: &UserID) -> Option<Vec<TaskID>> {
        let logs = self.logs.lock().unwrap();
        logs.get(user_id).map(|logs| logs.clone())
    }

    async fn accomplish_task(&self, user_id: &UserID, task: &TaskID) {
        let mut logs = self.logs.lock().unwrap();
        println!("Add task {} to user {}", task, user_id);
        let mut task_list = match logs.get(&user_id.to_string()) {
            Some(task_list) => task_list.clone(),
            None => Vec::new(),
        };
        task_list.push(task.to_string());
        logs.insert(user_id.to_string(), task_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_tasks() {
        let tasks = vec![
            Task {
                id: "1".into(),
                name: "Task 1".into(),
                description: "Description 1".into(),
            },
            Task {
                id: "2".into(),
                name: "Task 2".into(),
                description: "Description 2".into(),
            },
        ];
        let service = LocalTasksService {
            tasks: RwLock::new(tasks.clone()),
            logs: Default::default(),
        };
        let result = service.list_tasks().await;
        assert_eq!(result, tasks)
    }

    #[tokio::test]
    async fn create_task() {
        let task = Task {
            id: "1".into(),
            name: "Task 1".into(),
            description: "Description 1".into(),
        };
        let service = LocalTasksService::default();
        service.create_task(task.clone()).await;
        assert_eq!(service.list_tasks().await, vec![task])
    }
}
