pub type TaskID = u32;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: TaskID,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskInput {
    pub name: String,
    pub description: String,
}
