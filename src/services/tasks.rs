use crate::{
    model::{
        task::{Task, TaskID, TaskInput},
        user::UserID,
    },
    storage::TasksStorage,
};

#[tonic::async_trait]
pub trait TasksService {
    async fn list_tasks(&self) -> Vec<Task>;
    async fn create_task(&self, task_input: TaskInput) -> Task;
    async fn list_accomplished_tasks(&self, user_id: &UserID) -> Option<Vec<TaskID>>;
    async fn accomplish_task(&self, user_id: &UserID, task: &TaskID);
}

pub struct SimpleTasksService<Storage: TasksStorage> {
    storage: Storage,
}

impl<Storage: TasksStorage> SimpleTasksService<Storage> {
    pub fn new(storage: Storage) -> Self {
        SimpleTasksService { storage }
    }
}

#[tonic::async_trait]
impl<Storage: TasksStorage> TasksService for SimpleTasksService<Storage> {
    async fn list_tasks(&self) -> Vec<Task> {
        self.storage.list().await
    }

    async fn create_task(&self, task: TaskInput) -> Task {
        self.storage.create(task).await.unwrap()
    }

    async fn list_accomplished_tasks(&self, user_id: &UserID) -> Option<Vec<TaskID>> {
        self.storage.list_tasks_done(user_id).await
    }

    async fn accomplish_task(&self, user_id: &UserID, task: &TaskID) {
        self.storage.accomplish_task(user_id, task).await
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::postgres::service::{get_pool, PostgresStorage};

    use super::*;

    #[tokio::test]
    async fn list_tasks() {
        let pool = get_pool();
        let storage = PostgresStorage::new(pool);

        let tasks = vec![
            Task {
                id: 1,
                name: "Task 1".into(),
                description: "Description 1".into(),
            },
            Task {
                id: 2,
                name: "Task 2".into(),
                description: "Description 2".into(),
            },
        ];
        let service = SimpleTasksService { storage };
        let result = service.list_tasks().await;
        assert_eq!(result, tasks)
    }

    #[tokio::test]
    async fn create_task() {
        let pool = get_pool();
        let storage = PostgresStorage::new(pool);

        let task_input = TaskInput {
            name: "Task 1".into(),
            description: "Description 1".into(),
        };
        let expected_task = Task {
            id: 1,
            name: "Task 1".into(),
            description: "Description 1".into(),
        };

        let service = SimpleTasksService { storage };
        service.create_task(task_input.clone()).await;
        assert_eq!(service.list_tasks().await, vec![expected_task])
    }
}
