use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub date: String,
    pub done: bool,
}

pub type TaskList = Vec<Task>;
