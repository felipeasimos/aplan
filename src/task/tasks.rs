use std::collections::{HashMap, hash_map::Values};

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::prelude::{TaskId, Error};

use super::{Task, TaskStatus};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tasks {
    #[serde_as(as = "Vec<(_, _)>")]
    tasks: HashMap<TaskId, Task>
}

impl Tasks {
    pub(crate) fn new() -> Self {
        Self {
            tasks: HashMap::new()
        }
    }

    pub fn get(&self, task_id: &TaskId) -> Result<&Task, Error> {
        self.tasks.get(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }

    pub fn tasks(&self) -> Values<TaskId, Task> {
        self.tasks.values()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn leaf_tasks(&self) -> impl Iterator<Item=&Task> {
        self
            .tasks()
            .filter(|task| task.is_leaf())
    }

    pub fn todo_tasks(&self) -> impl Iterator<Item=&Task> {
        self
            .leaf_tasks()
            .filter(|task| task.status != TaskStatus::Done)
    }

    pub fn in_progress_tasks(&self) -> impl Iterator<Item=&Task> {
        self
            .leaf_tasks()
            .filter(|task| task.status == TaskStatus::InProgress)
    }

    pub fn done_tasks(&self) -> impl Iterator<Item=&Task> {
        self
            .leaf_tasks()
            .filter(|task| task.status == TaskStatus::Done)
    }

    pub(crate) fn get_mut(&mut self, task_id: &TaskId) -> Result<&mut Task, Error> {
        self.tasks.get_mut(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }

    pub(crate) fn insert(&mut self, task_id: TaskId, task: Task) {
        self.tasks.insert(task_id, task);
    }

    pub(crate) fn remove(&mut self, task_id: &TaskId) -> Result<Task, Error> {
        self.tasks.remove(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }
}
