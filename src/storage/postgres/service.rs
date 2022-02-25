use diesel::{r2d2::ConnectionManager, PgConnection, RunQueryDsl};
use r2d2::Pool;

use super::schema::tasks::dsl::*;
use super::task::Task;
use crate::model::task as model;
use crate::model::user::UserID;
use crate::storage::postgres::task::NewTask;
use crate::storage::{StorageResult, TasksStorage};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PgPool {
    let database_url = "postgres://tasks:example@localhost:5433/tasks";
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder().max_size(5).build(manager).unwrap()
}

pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub fn new(pool: PgPool) -> Self {
        PostgresStorage { pool }
    }
}

#[tonic::async_trait]
impl TasksStorage for PostgresStorage {
    async fn list(&self) -> Vec<model::Task> {
        let connection = self.pool.get().unwrap();

        let results = tasks
            .load::<Task>(&connection)
            .expect("Error loading posts");

        results.into_iter().map(|task| task.into()).collect()
    }

    async fn create(&self, task: model::TaskInput) -> StorageResult<model::Task> {
        let connection = self.pool.get().unwrap();
        let input: NewTask = task.into();

        println!("Input: {:?}", input);

        let task: Task = diesel::insert_into(tasks)
            .values(&input)
            .get_result(&connection)
            .unwrap();

        Ok(task.into())
    }

    async fn list_tasks_done(&self, _user_id: &UserID) -> Option<Vec<model::TaskID>> {
        todo!()
    }

    async fn accomplish_task(
        &self,
        _user_id: &crate::model::user::UserID,
        _task: &crate::model::task::TaskID,
    ) -> () {
        todo!()
    }
}
