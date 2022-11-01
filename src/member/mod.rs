pub(crate) mod members;

use std::collections::{HashSet, HashMap};
use std::fmt::Display;

use chrono::{Utc, DateTime, NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::task::task_id::TaskId;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    name: String,
    weekly_cost: f64,
    #[serde_as(as = "HashSet<_>")]
    tasks: HashSet<TaskId>,
    #[serde_as(as = "Vec<(_, _)>")]
    routine_exceptions: HashMap<NaiveDateTime, f64>
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

    fn to_datetime(date: &NaiveDate) -> NaiveDateTime {
        date.and_hms(0, 0, 0)
    }

    pub(crate) fn add_routine_exception(&mut self, date: &NaiveDate, cost: f64) {
        self.routine_exceptions.insert(Member::to_datetime(date), cost);
    }

    pub(crate) fn remove_routine_exception(&mut self, date: &NaiveDate) {
        self.routine_exceptions.remove(&Member::to_datetime(date));
    }

    pub(crate) fn get_routine_exception(&self, date: &NaiveDate) -> Option<&f64> {
        self.routine_exceptions.get(&Member::to_datetime(date))
    }
}

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tasks = self.task_ids().fold(String::new(), |acc, id| acc + &id.to_string() + " ");
        let tasks = tasks.trim_end();
        write!(f, "{} - [{}]", self.name, tasks)
    }
}
