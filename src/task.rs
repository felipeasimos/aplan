use std::{ops::Range, option::Iter};

use crate::task_id::TaskId;

#[derive(Clone, Debug)]
pub struct Task {
    name: String,
    pub(crate) id: TaskId,
    pub(crate) planned_value: f64,
    pub(crate) actual_cost: f64,
    pub(crate) num_child: u32
}

impl Eq for Task {}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Task {
    pub(crate) fn new(id: TaskId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            planned_value: 0.0,
            actual_cost: 0.0,
            num_child: 0
        }
    }

    pub fn id(&self) -> &TaskId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn set_id(&mut self, id: TaskId) {
        self.id = id;
    }

    pub fn get_planned_value(&self) -> f64 {
        self.planned_value
    }

    pub fn get_actual_cost(&self) -> f64 {
        self.actual_cost
    }

    pub fn child_ids(&self) -> Vec<TaskId> {
        self.id().child_ids(self.num_child)
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        match self.id().as_vec().last() {
            Some(_) => format!("{} - {}\npv: {}, ac: {}", self.id().to_string(), self.name(), self.planned_value, self.actual_cost),
            None => format!("{}\npv: {}, ac: {}", self.name().to_string(), self.planned_value, self.actual_cost),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::task_id::*;

    use super::*;

    #[test]
    fn create_task() {
        let task_id = TaskId::parse("1.1").unwrap();
        let task = Task::new(task_id.clone(), "Create Task Struct");
        assert_eq!(task.id(), &task_id);
    }
}
