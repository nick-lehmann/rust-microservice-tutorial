use crate::diesel::ExpressionMethods;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

use super::schema::task_logs::dsl::*;
use super::schema::tasks::dsl::*;
use super::task::{NewTaskLog, Task};
use crate::model::task as model;
use crate::model::user as user_model;
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
            .filter(id.eq(1))
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

    async fn list_tasks_done(&self, user: &user_model::UserID) -> Option<Vec<model::TaskID>> {
        let connection = self.pool.get().unwrap();

        let user_id_int: i32 = user.parse().unwrap();

        let result: Vec<i32> = tasks
            .inner_join(task_logs)
            .select(task_id)
            .filter(user_id.eq(user_id_int))
            .load(&connection)
            .unwrap();

        Some(result.into_iter().map(|x| x as u32).collect())
    }

    async fn accomplish_task(
        &self,
        user: &user_model::UserID,
        task: &crate::model::task::TaskID,
    ) -> () {
        let connection = self.pool.get().unwrap();
        let user_id_integer: i32 = user.parse().unwrap();

        let input = NewTaskLog {
            task_id: task.to_owned() as i32,
            user_id: user_id_integer,
        };

        diesel::insert_into(task_logs)
            .values(input)
            .execute(&connection)
            .unwrap();
    }
}
