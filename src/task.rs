use crate::{task_id::TaskId, to_dot_str::ToDotStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    id: TaskId,
    name: String,
}

impl Task {
    pub(crate) fn new(id: TaskId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
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
}

impl ToDotStr for Task {
    fn to_dot_str(&self) -> String {
        match self.id().as_vec().last() {
            Some(_) => format!("\"{} - {}\"", self.id().to_string(), self.name()),
            None => format!("\"{}\"", self.name()),
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
