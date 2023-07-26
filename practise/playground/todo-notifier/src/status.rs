use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub completed: bool,
    pub id: u16,
}

impl Status {
    pub fn new(completed: bool, id: u16) -> Self {
        Self {
            completed,
            id
        }
    }
}