use super::schema::{task_logs, tasks};
use crate::model::task as model;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl From<Task> for model::Task {
    fn from(val: Task) -> Self {
        model::Task {
            id: val.id as u32,
            name: val.name,
            description: val.description,
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct NewTask {
    pub name: String,
    pub description: String,
}

impl From<model::TaskInput> for NewTask {
    fn from(task: model::TaskInput) -> Self {
        NewTask {
            name: task.name,
            description: task.description,
        }
    }
}

#[derive(Insertable)]
#[table_name = "task_logs"]
pub struct NewTaskLog {
    pub task_id: i32,
    pub user_id: i32,
}
