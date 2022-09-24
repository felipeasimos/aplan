use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{task::Task, task_id::TaskId, wsb::WSB, task_store::TaskStore};

pub struct Project {

    store: TaskStore,
    name: String,
    wsb: WSB,
    // burndown: Burndown
}

impl Project {
    pub fn new(name: &str) -> Self {
        let store = TaskStore::new();
        Self {
            store: store.clone(),
            name: name.to_string(),
            wsb: WSB::new(name, &store)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_project() {

        let project = Project::new("Create Aplan");
        assert_eq!(project.name(), "Create Aplan");
    }
}
