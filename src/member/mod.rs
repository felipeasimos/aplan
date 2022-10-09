use std::collections::{hash_set::Iter, HashSet};

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::task::task_id::TaskId;
use crate::subsystem::schedule::Schedule;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    name: String,
    // schedule: Schedule,
    #[serde_as(as = "HashSet<_>")]
    tasks: HashSet<TaskId>
}

impl Member {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            // schedule: Schedule::new(),
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
