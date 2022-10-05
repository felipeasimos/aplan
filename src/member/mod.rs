pub mod members;

use serde::{Serialize, Deserialize};

use crate::subsystem::schedule::Schedule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    name: String,
    schedule: Schedule
}

impl Member {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            schedule: Schedule::new()
        }
    }
}
