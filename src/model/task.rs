pub type TaskID = String;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: TaskID,
    pub name: String,
    pub description: String,
}
