use serde::{Serialize, Deserialize};

use super::schedule::Schedule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    name: String,
    schedule: Schedule
}

impl Member {
    fn new(name: String) -> Self {
        Self {
            name,
            schedule: Schedule::new()
        }
    }
}
