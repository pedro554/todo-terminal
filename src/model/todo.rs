use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    Backlog,
    InProgress,
    Completed,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub state: State,
}
