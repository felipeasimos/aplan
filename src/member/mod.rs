pub(crate) mod members;

use std::collections::{HashSet, HashMap};
use std::fmt::Display;

use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::task::task_id::TaskId;

type Cost = f64;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    name: String,
    weekly_cost: Cost,
    #[serde_as(as = "HashSet<_>")]
    tasks: HashSet<TaskId>,
    #[serde_as(as = "Vec<(_, _)>")]
    routine_exceptions: HashMap<DateTime<Utc>, Cost>
}

impl Member {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            weekly_cost: 0.0,
            tasks: HashSet::new(),
            routine_exceptions: HashMap::new()
        }
    }

    pub fn is_assigned_to(&self, task_id: &TaskId) -> bool {
        self.tasks.contains(task_id)
    }

    pub fn task_ids(&self) -> impl Iterator<Item=&TaskId> + '_ {
        self.tasks.iter()
    }

    pub(crate) fn add_task(&mut self, task_id: TaskId) {
        self.tasks.insert(task_id);
    }

    pub(crate) fn remove_task(&mut self, task_id: &TaskId) {
        self.tasks.remove(task_id);
    }
}

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tasks = self.task_ids().fold(String::new(), |acc, id| acc + &id.to_string() + " ");
        let tasks = tasks.trim_end();
        write!(f, "{} - [{}]", self.name, tasks)
    }
}
