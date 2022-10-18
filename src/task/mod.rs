pub mod task_id;
pub(crate) mod tasks;

use std::{fmt::Display, collections::HashSet};

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use self::task_id::TaskId;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskStatus {
    InProgress,
    Done
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::InProgress => write!(f, "InProgress"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

impl TaskStatus {
    fn to_icon(&self) -> &'static str {
        match &self {
            TaskStatus::InProgress => "✗",
            TaskStatus::Done => "✔"
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    name: String,
    pub(crate) id: TaskId,
    pub(crate) planned_value: f64,
    pub(crate) actual_cost: f64,
    pub(crate) num_child: u32,
    pub(crate) status: TaskStatus,
    #[serde_as(as = "HashSet<_>")]
    members_names: HashSet<String>
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
            num_child: 0,
            status: TaskStatus::InProgress,
            members_names: HashSet::new()
        }
    }

    pub fn id(&self) -> &TaskId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_planned_value(&self) -> f64 {
        self.planned_value
    }

    pub fn get_actual_cost(&self) -> f64 {
        self.actual_cost
    }

    pub fn child_ids(&self) -> impl Iterator<Item=TaskId> + '_ {
        self.id().child_ids(self.num_child)
    }

    pub fn is_trunk(&self) -> bool {
        self.num_child > 0
    }

    pub fn is_leaf(&self) -> bool {
        self.num_child == 0
    }

    pub(crate) fn add_member(&mut self, name: &str) {
        self.members_names.insert(name.to_string());
    }

    pub(crate) fn remove_member(&mut self, name: &str) {
        self.members_names.remove(name);
    }

    pub fn has_member(&self, name: &str) -> bool {
        self.members_names.contains(name)
    }

    pub fn to_dot_str(&self) -> String {
        let members = self.members_names.iter().fold(String::new(), |acc, name| acc + name + " ");
        let members = members.trim_end();
        format!(
            "{} - {} {}\npv: {} ac: {}\n[{}]",
            self.id().to_string(),
            self.name(),
            self.status.to_icon(),
            self.get_planned_value(),
            self.get_actual_cost(),
            members)
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let members = self.members_names.iter().fold(String::new(), |acc, name| acc + name + " ");
        let members = members.trim_end();
        match self.id().as_vec().last() {
            Some(_) => write!(f, "{} - {} {} - [{}]", self.id().to_string(), self.name(), self.status.to_icon(), members),
            None => write!(f, "{} {}", self.name().to_string(), self.status.to_icon()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task() {
        let task_id = TaskId::parse("1.1").unwrap();
        let task = Task::new(task_id.clone(), "Create Task Struct");
        assert_eq!(task.id(), &task_id);
    }
}
