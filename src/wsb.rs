use std::{collections::HashMap, rc::Rc, cell::{RefCell, Ref, RefMut}, pin::Pin, path::PathBuf};

use trees::{Tree, Node};
use std::io::{Write};

use crate::{task::Task, task_id::TaskId, task_store::TaskStore, to_dot_str::ToDotStr};

#[derive(Debug)]
pub struct WSB {

    store: TaskStore,
    tree: Tree<TaskId>
}

impl WSB {

    fn get_root_id() -> TaskId {
        TaskId::new(vec![])
    }

    pub fn new(name: &str, store: &TaskStore) -> Self {
        let root_id = Self::get_root_id();
        let root_task = Task::new(root_id.clone(), name);
        store.insert(root_id, root_task);
        Self {
            store: store.clone(),
            tree: Tree::new(Self::get_root_id())
        }
    }

    pub fn get_task_node(&self, id: &TaskId) -> Option<&Node<TaskId>> {
        let root = self.tree.root();
        if root.data() == id {
            return Some(root);
        }
        // start iterating from the root's children
        let node = id.as_vec().iter().enumerate().try_fold(root, |node, (depth, layer_id)| {
            // for each node, get the child associated with the id
            let child = node.iter().find(|child| child.data().as_vec()[depth] == *layer_id)?;
            Some(child)
        })?;
        Some(node)
    }

    pub fn get_task_node_mut(&mut self, id: &TaskId) -> Option<&mut Node<TaskId>> {
        let root = self.tree.root_mut().get_mut();
        if root.data() == id {
            return Some(root);
        }
        // start iterating from the root's children
        let node = id.as_vec().iter().enumerate().try_fold(root, |node, (depth, layer_id)| {
            // for each node, get the child associated with the id
            let child = node.iter_mut().find(|child| child.data().as_vec()[depth] == *layer_id)?;
            Some(child.get_mut())
        })?;
        Some(node)
    }

    pub fn get_task(&self, id: &str) -> Option<Ref<Task>> {
        let task_id = TaskId::parse(id)?;
        self.store.get(&task_id)
    }

    pub fn get_task_mut(&mut self, id: &str) -> Option<RefMut<Task>> {
        let task_id = TaskId::parse(id)?;
        self.store.get_mut(&task_id)
    }

    pub fn add_task(&mut self, parent_id: &str, name: &str) -> Option<RefMut<Task>> {
        // add implict root id
        let parent_task_id = TaskId::parse(parent_id)?;

        let mut task_id_vec = parent_task_id.as_vec().clone();
        let parent_node = self.get_task_node_mut(&parent_task_id)?;

        let new_layer_id : u32 = parent_node.degree().try_into().ok()?;
        task_id_vec.push(new_layer_id + 1u32);
        let task_id = TaskId::new(task_id_vec);
        let task = Task::new(task_id.clone(), name);

        // add task_id to the node
        parent_node.push_back(Tree::new(task_id.clone()));

        // add task to task store
        self.store.insert(task_id.clone(), task);

        self.get_task_mut(&task_id.to_string())
    }

    pub fn expand<const N: usize>(&mut self, arr: &[(&str, &str); N]) -> Option<&mut Self> {
        for (parent_id, task_name) in arr {
            self.add_task(parent_id, task_name)?;
        }
        Some(self)
    }

    fn subtract_id(store_rc: TaskStore, child: &mut Node<TaskId>, layer_idx: usize) {
        let old_task_id = child.data().clone();
        child.data_mut().as_vec_mut()[layer_idx] -= 1;
        store_rc.copy(&old_task_id, child.data().clone());

        child.iter_mut().for_each(|node| {
            Self::subtract_id(store_rc.clone(), node.get_mut(), layer_idx);
        });
    }

    fn apply_along_path<F: Fn(RefMut<Task>)>(&mut self, id: &TaskId, func: F) -> Option<()> {
        let root = self.tree.root();
        func(self.store.get_mut(&Self::get_root_id())?);
        if root.data() == id {
            return Some(());
        }
        // start iterating from the root's children
        id.as_vec().iter().enumerate().try_fold(root, |node, (depth, layer_id)| {
            // for each node, get the child associated with the id
            let child : &Node<TaskId> = node.iter().find(|child| child.data().as_vec()[depth] == *layer_id)?;
            func(self.store.get_mut(child.data())?);
            Some(child)
        })?;
        Some(())
    }

    pub fn remove(&mut self, id: &str) -> Option<Task> {
        let mut task_id = TaskId::parse(id)?;

        {
            // remove node from tree
            let task_node = self.get_task_node_mut(&task_id)?;
            // can't remove trunk nodes
            if task_node.degree() > 0 {
                return None;
            }
            task_node.detach().root_mut().data();
        }

        let task = self.store.remove(&task_id);

        let parent_id = task_id.parent()?;
        let store_rc = self.store.clone();
        let parent_node = self.get_task_node_mut(&parent_id)?;
        let layer_idx = task_id.as_vec().len() - 1;
        let child_idx = (*task_id.as_vec().last()? as usize) - 1;

        // change id of child that comes after id node
        parent_node.iter_mut().enumerate().for_each(|(index, mut child)| {

            if child_idx <= index {
                Self::subtract_id(store_rc.clone(), &mut child, layer_idx);
            }
        });

        // remove last id from the store
        task_id.as_vec_mut()[layer_idx] = parent_node.degree() as u32 + 1;
        self.store.remove(&task_id);

        // // remove planned value
        // let planned_value_to_remove = task.clone()?.get_planned_value();
        // self.apply_along_path(&parent_id, |t| {
        //     t.planned_value -= planned_value_to_remove
        // });

        task
    }

    fn subtree_to_dot_str(&self, root: &Node<TaskId>) -> String {
        let mut s = String::new();
        let root_str = self.store.get(root.data()).unwrap().to_dot_str();
        root.iter().for_each(|node| {
            s += &format!("\t{} -> {}\n", root_str, self.store.get(node.data()).unwrap().to_dot_str());
            s += &self.subtree_to_dot_str(node);
        });
        s
    }
}

impl ToDotStr for WSB {
    fn to_dot_str(&self) -> String {
        "digraph G {\n".to_string() +
            &self.subtree_to_dot_str(self.tree.root()) +
            &"}".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tasks() {
        let store = TaskStore::new();
        let mut wsb = WSB::new("Project", &store);

        assert!(wsb.add_task("1", "Create WSB").is_none());
        assert_eq!(wsb.add_task("", "Create WSB").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(wsb.add_task("1", "Create Task struct").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        wsb.expand(&[
            ("", "Create CLI tool"),
                ("2", "Create argument parser"),
                ("2", "Create help menu"),
            ("", "Create GUI tool"),
                ("3", "Create plot visualizer")
        ]);
        assert_eq!(wsb.get_task("1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(wsb.get_task_mut("1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1]), "Create WSB")));

        assert_eq!(wsb.get_task("1.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        assert_eq!(wsb.get_task_mut("1.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        // assert_eq!(wsb.set_planned_value("1.1", 2.0), Some(()));
        // assert_eq!(wsb.get_planned_value(), 2.0);
        // assert_eq!(wsb.get_task("1.1").unwrap().get_planned_value(), 2.0);
        // assert_eq!(wsb.get_task("1").unwrap().get_planned_value(), 2.0);

        assert_eq!(wsb.get_task("2").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        assert_eq!(wsb.get_task_mut("2").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2]), "Create CLI tool")));

        assert_eq!(wsb.get_task("2.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2,1]), "Create argument parser")));
        assert_eq!(wsb.get_task_mut("2.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2,1]), "Create argument parser")));
        // assert_eq!(wsb.set_planned_value("2.1", 7.0), Some(()));
        // assert_eq!(wsb.get_planned_value(), 9.0);
        // assert_eq!(store.get_task("2.1").unwrap().get_planned_value(), 7.0);
        // assert_eq!(store.get_task("2.2").unwrap().get_planned_value(), 0.0);
        // assert_eq!(store.get_task("2").unwrap().get_planned_value(), 7.0);

        assert_eq!(wsb.get_task("2.2").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2,2]), "Create help menu")));
        assert_eq!(wsb.get_task_mut("2.2").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2,2]), "Create help menu")));
        // assert_eq!(store.set_planned_value("2.2", 33.0), Some(()));
        // assert_eq!(store.get_planned_value(), 42.0);
        // assert_eq!(store.get_task("2.1").unwrap().get_planned_value(), 7.0);
        // assert_eq!(store.get_task("2.2").unwrap().get_planned_value(), 33.0);
        // assert_eq!(store.get_task("2").unwrap().get_planned_value(), 40.0);

        assert_eq!(wsb.get_task("3").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![3]), "Create GUI tool")));
        assert_eq!(wsb.get_task_mut("3").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![3]), "Create GUI tool")));

        assert_eq!(wsb.get_task("3.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![3,1]), "Create plot visualizer")));
        assert_eq!(wsb.get_task_mut("3.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![3,1]), "Create plot visualizer")));
        // assert_eq!(store.set_planned_value("3.1", 20.0), Some(()));
        // assert_eq!(store.get_planned_value(), 62.0);
        // assert_eq!(store.get_task("3.1").unwrap().get_planned_value(), 20.0);
        // assert_eq!(store.get_task("3").unwrap().get_planned_value(), 20.0);
        assert_eq!(wsb.remove("2.1"), Some(Task::new(TaskId::new(vec![2,1]), "Create argument parser")));
        // assert_eq!(store.get_planned_value(), 55.0);
        assert_eq!(wsb.get_task("2.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2, 1]), "Create help menu")));
        assert_eq!(wsb.get_task("2").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        // assert_eq!(store.get_task("2").unwrap().get_planned_value(), 33.0);
        //
        println!("{:#?}", wsb);
        assert_eq!(wsb.remove("2"), None);
        // assert_eq!(store.get_planned_value(), 55.0);
        assert_eq!(wsb.remove("2.1"), Some(Task::new(TaskId::new(vec![2,1]), "Create help menu")));
        // assert_eq!(store.get_planned_value(), 22.0);
        // assert_eq!(store.get_task("2").unwrap().get_planned_value(), 0.0);
        assert_eq!(wsb.remove("2"), Some(Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        // assert_eq!(store.get_planned_value(), 22.0);

        assert_eq!(wsb.get_task("1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(wsb.get_task_mut("1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1]), "Create WSB")));

        assert_eq!(wsb.get_task("1.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        assert_eq!(wsb.get_task_mut("1.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![1,1]), "Create Task struct")));

        assert_eq!(wsb.get_task("2").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2]), "Create GUI tool")));
        assert_eq!(wsb.get_task_mut("2").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2]), "Create GUI tool")));

        assert_eq!(wsb.get_task("2.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2,1]), "Create plot visualizer")));
        assert_eq!(wsb.get_task_mut("2.1").map(|v| v.clone()), Some(Task::new(TaskId::new(vec![2,1]), "Create plot visualizer")));
        // write!(std::fs::File::create("test").unwrap(), "{}", wsb.to_dot_str());
    }
}
