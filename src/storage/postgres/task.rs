use super::schema::tasks;
use crate::model::task as model;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Into<model::Task> for Task {
    fn into(self) -> model::Task {
        model::Task {
            id: self.id as u32,
            name: self.name,
            description: self.description,
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
