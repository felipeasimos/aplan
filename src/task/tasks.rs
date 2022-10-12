use std::collections::{HashMap, hash_map::Values};

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::prelude::{TaskId, Error};

use super::Task;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Tasks {
    #[serde_as(as = "Vec<(_, _)>")]
    tasks: HashMap<TaskId, Task>
}

impl Tasks {
    pub(crate) fn new() -> Self {
        Self {
            tasks: HashMap::new()
        }
    }

    pub(crate) fn insert(&mut self, task_id: TaskId, task: Task) {
        self.tasks.insert(task_id, task);
    }

    pub(crate) fn get(&self, task_id: &TaskId) -> Result<&Task, Error> {
        self.tasks.get(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }

    pub(crate) fn get_mut(&mut self, task_id: &TaskId) -> Result<&mut Task, Error> {
        self.tasks.get_mut(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }

    pub(crate) fn len(&self) -> usize {
        self.tasks.len()
    }

    pub(crate) fn remove(&mut self, task_id: &TaskId) -> Result<Task, Error> {
        self.tasks.remove(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }

    pub(crate) fn tasks(&self) -> Values<TaskId, Task> {
        self.tasks.values()
    }
}
