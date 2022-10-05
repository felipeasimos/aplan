pub mod members;

use std::collections::{HashSet, hash_set::Iter};

use serde::{Serialize, Deserialize};

use crate::{subsystem::schedule::Schedule, task::task_id::TaskId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    name: String,
    schedule: Schedule,
    tasks: HashSet<TaskId>
}

impl Member {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            schedule: Schedule::new(),
            tasks: HashSet::new()
        }
    }

    pub fn add_task(&mut self, task_id: TaskId) {
        self.tasks.insert(task_id);
    }

    pub fn remove_task(&mut self, task_id: &TaskId) {
        self.tasks.remove(task_id);
    }

    pub fn tasks(&self) -> Iter<'_, TaskId> {
        self.tasks.iter()
    }
}
