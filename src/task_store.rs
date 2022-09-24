use std::{collections::HashMap, cell::{RefCell, Ref, RefMut}, rc::Rc};

use crate::{task::Task, task_id::TaskId};

#[derive(Debug)]
pub struct TaskStore {

    store: Rc<RefCell<HashMap<TaskId, Task>>>
}

impl Clone for TaskStore {
    fn clone(&self) -> Self {
        Self {
            store: Rc::clone(&self.store)
        }
    }
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn get(&self, id: &TaskId) -> Option<Ref<Task>> {
        Ref::filter_map(self.store.borrow(), |v| v.get(id)).ok()
    }

    pub fn get_mut(&self, id: &TaskId) -> Option<RefMut<Task>> {
        RefMut::filter_map(self.store.borrow_mut(), |v| v.get_mut(id)).ok() 
    }

    pub fn remove(&self, id: &TaskId) -> Option<Task> {
        self.store.borrow_mut().remove(id)
    }

    pub fn insert(&self, id: TaskId, task: Task) {
        self.store.borrow_mut().insert(id, task);
    }

    pub fn copy(&self, from: &TaskId, to: TaskId) -> Option<()> {
        let mut data = self.get(from)?.clone();
        data.set_id(to.clone());
        self.insert(to.clone(), data);
        Some(())
    }
}

#[cfg(test)]
mod tests {

}
