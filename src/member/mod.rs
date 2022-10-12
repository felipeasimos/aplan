pub(crate) mod members;

use std::collections::HashSet;
use std::fmt::Display;

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::task::task_id::TaskId;
use crate::subsystem::schedule::Schedule;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    name: String,
    schedule: Schedule,
    #[serde_as(as = "HashSet<_>")]
    tasks: HashSet<TaskId>
}

impl Member {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            schedule: Schedule::new(),
            tasks: HashSet::new()
        }
    }

    pub(crate) fn add_task(&mut self, task_id: TaskId) {
        self.tasks.insert(task_id);
    }

    pub(crate) fn remove_task(&mut self, task_id: &TaskId) {
        self.tasks.remove(task_id);
    }

    pub(crate) fn task_ids(&self) -> impl Iterator<Item=&TaskId> + '_ {
        self.tasks.iter()
    }
}

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tasks = self.task_ids().fold(String::new(), |acc, id| acc + &id.to_string() + " ");
        let tasks = tasks.trim_end();
        write!(f, "{} - [{}]", self.name, tasks)
    }
}
