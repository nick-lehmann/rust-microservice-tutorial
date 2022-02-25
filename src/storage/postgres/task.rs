use super::schema::tasks;
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
