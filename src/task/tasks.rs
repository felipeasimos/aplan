use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::serde_as;

use crate::prelude::{TaskId, Error, Members};

use super::{Task, TaskStatus};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tasks {
    #[serde_as(as="Vec<(_, _)>")]
    store: HashMap<TaskId, Task>
}

impl Tasks {
    pub(crate) fn new(name: &str) -> Self {
        let mut store = HashMap::new();
        let root_id = TaskId::get_root_id();
        let root_task = Task::new(root_id.clone(), name);
        store.insert(root_id, root_task);
        Self {
            store
        }
    }

    /// SAFETY: uses `unwrap` instead of returning an error because a root node should always
    /// exists
    pub fn name(&self) -> &str {
        self.get(&TaskId::get_root_id()).unwrap().name()
    }

    /// SAFETY: uses `unwrap` instead of returning an error because a root node should always
    /// exists
    pub fn planned_value(&self) -> f64 {
        self.get(&TaskId::get_root_id()).unwrap().get_planned_value()
    }

    /// SAFETY: uses `unwrap` instead of returning an error because a root node should always
    /// exists
    pub fn actual_cost(&self) -> f64 {
        self.get(&TaskId::get_root_id()).unwrap().get_actual_cost()
    }

    pub fn completion_percentage(&self) -> f64 {
        self.get_done_tasks().count() as f64 / self.len() as f64
    }

    pub fn earned_value(&self) -> f64 {
        self.planned_value() * self.completion_percentage()
    }

    pub fn spi(&self) -> f64 {
        let res = self.earned_value() / self.planned_value();
        if res.is_nan() {
            0.0
        } else {
            res
        }
    }

    pub fn sv(&self) -> f64 {
        self.earned_value() - self.planned_value()
    }

    pub fn cpi(&self) -> f64 {
        let res = self.earned_value() / self.actual_cost();
        if res.is_nan() {
            0.0
        } else {
            res
        }
    }

    pub fn cv(&self) -> f64 {
        self.earned_value() - self.actual_cost()
    }

    pub(crate) fn remove_task(&mut self, task_id: &TaskId) -> Result<Task, Error> {
        self.store.remove(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }

    pub(crate) fn add_dependency(&mut self, task_id: &TaskId, dependency_id: &TaskId) -> Result<(), Error> {
        {
            self.get_mut(task_id)?;
            self.get_mut(dependency_id)?;
        }
        // SAFETY: we already performed `get_mut`, so we know these exist
        self.store.get_mut(task_id).unwrap().dependencies.insert(dependency_id.clone());
        self.store.get_mut(dependency_id).unwrap().dependency_for.insert(task_id.clone());
        Ok(())
    }

    pub(crate) fn remove_dependency(&mut self, task_id: &TaskId, dependency_id: &TaskId) -> Result<(), Error> {
        if !self.get_mut(task_id)?.dependencies.contains(dependency_id) {
            return Err(Error::TaskNotFound(dependency_id.clone()));
        }
        if !self.get_mut(dependency_id)?.dependency_for.contains(task_id) {
            return Err(Error::TaskNotFound(task_id.clone()));
        }
        // SAFETY: we already performed `get_mut`, so we know these exist
        self.get_mut(task_id).unwrap().dependencies.remove(dependency_id);
        self.get_mut(dependency_id).unwrap().dependency_for.remove(task_id);

        Ok(())
    }

    pub fn next_sibling(&self, task_id: &TaskId) -> Result<&Task, Error> {
        let next_sibling_id = task_id.next_sibling()?;
        self.get(&next_sibling_id)
            .map_err(|_| Error::NoNextSibling(task_id.clone()))
    }

    pub fn prev_sibling(&self, task_id: &TaskId) -> Result<&Task, Error> {
        let prev_sibling_id = task_id.prev_sibling()?;
        self.get(&prev_sibling_id)
            .map_err(|_| Error::NoPrevSibling(task_id.clone()))
    }

    pub(crate) fn add_task(&mut self, parent_task_id: TaskId, name: &str) -> Result<&mut Task, Error> {
        // get parent
        let parent_task = self.get_mut(&parent_task_id)?;

        // increase number of children
        parent_task.num_child += 1;

        // get new task id
        let task_id = parent_task_id.new_child_id(parent_task.num_child)?;

        // create task
        let task = Task::new(task_id.clone(), name);

        // add task to task map
        self.insert(task_id.clone(), task);

        // since new tasks are always not done, all parents must be not done too
        self.apply_along_path(&task_id, |task| {
            task.status = TaskStatus::InProgress;
        })?;

        self.get_mut(&task_id)
    }

    pub(crate) fn expand<const N: usize>(&mut self, arr: &[(&str, &str); N]) -> Result<&mut Self, Error> {
        for (parent_id, task_name) in arr {
            self.add_task(TaskId::parse(parent_id)?, task_name)?;
        }
        Ok(self)
    }

    fn apply_along_path<F: Fn(&mut Task)>(&mut self, id: &TaskId, func: F) -> Result<(), Error> {
        id
            .path()
            .try_for_each(|id| {
                let child = self.get_mut(&id)?;
                func(child);
                Ok(())
            })
    }

    fn subtract_id(&mut self, child_id: &TaskId, layer_idx: usize) -> Result<(), Error> {
        let num_child = self.get(child_id)?.num_child;
        let old_task_id = child_id.clone();
        let mut new_task_id = child_id.clone();
        new_task_id.as_vec_mut()[layer_idx] -= 1;
        let mut task = self.remove_task(&old_task_id)?;
        task.id = new_task_id.clone();
        self.insert(
            new_task_id,
            task
        );

        child_id.child_ids(num_child).try_for_each(|node_id| {
            self.subtract_id(&node_id, layer_idx)
        })
    }

    pub(crate) fn remove(&mut self, task_id: &TaskId, members: &Members) -> Result<Task, Error> {
        // don't remove if this is a trunk node
        dbg!(&task_id);
        if self.get(&task_id)?.num_child > 0 {
            return Err(Error::TrunkCannotBeRemoved(task_id.clone()));
        }
        // task can't be removed if there are members assigned to it
        if members.members().any(|member| member.is_assigned_to(&task_id)) {
            return Err(Error::CannotRemoveAssignedTask(task_id.clone()))
        }

        self.remove_task_stats_from_tree(&task_id)?;

        let parent_id = task_id.parent()?;
        let parent_childs: _ = {
            let mut parent = self.get_mut(&parent_id)?;
            parent.num_child -= 1;
            parent.id()
                .child_ids(parent.num_child+1)
                .collect::<Vec<TaskId>>()
        };

        let layer_idx = task_id.len() - 1;
        let child_idx = task_id.child_idx()? as usize - 1;

        let task = self.remove_task(&task_id)?;

        // change id of child that comes after id node
        parent_childs.iter().enumerate().try_for_each(|(index, child_id)| -> Result<(), _> {
            if child_idx < index {
                self.subtract_id(&child_id, layer_idx)?;
            }
            Ok(())
        })?;

        Ok(task)
    }

    fn remove_task_stats_from_tree(&mut self, task_id: &TaskId) -> Result<(), Error> {

        self.set_actual_cost(&task_id, 0.0)?;
        self.set_planned_value(&task_id, 0.0)?;
        Ok(())
    }

    fn children_are_done(&self, task_id: &TaskId) -> bool {
        self.get(task_id).unwrap()
            .child_ids()
            .find(|id| self.get(id).unwrap().status != TaskStatus::Done)
            .is_none()
    }

    pub(crate) fn set_actual_cost(&mut self, task_id: &TaskId, actual_cost: f64) -> Result<(), Error> {
        let parent_id = task_id.parent()?;
        {
            let mut task = self.get_mut(&task_id)?;
            if task.is_trunk() {
                return Err(Error::TrunkCannotChangeCost(task_id.clone()));
            }
            let old_actual_cost = task.actual_cost;
            task.actual_cost = actual_cost;
            let diff = actual_cost - old_actual_cost;

                self.apply_along_path(&parent_id, |mut task| {
                    task.actual_cost += diff;
                })?;
        }

        task_id
            .clone()
            .path()
            .rev()
            .try_for_each(|id| {
                if self.children_are_done(&id) {
                    self.get_mut(&id)?.status = TaskStatus::Done;
                }
                Ok(())
            })
    }

    pub(crate) fn set_planned_value(&mut self, task_id: &TaskId, planned_value: f64) -> Result<(), Error> {
        let parent_id = task_id.parent()?;
        let mut task = self.get_mut(&task_id)?;
        // can't set actual cost of trunk node
        if task.is_trunk() {
            return Err(Error::TrunkCannotChangeValue(task_id.clone()));
        }
        let old_planned_value = task.planned_value;
        task.planned_value = planned_value;
        let diff = planned_value - old_planned_value;

        self.apply_along_path(&parent_id, |mut task| {
            task.planned_value += diff;
        })
    }

    pub fn to_dot_str(&self) -> String {
        let stats = format!(
            "earned value: {}, spi: {}, sv: {}, cpi: {}, cv: {}",
            self.earned_value(),
            self.spi(),
            self.sv(),
            self.cpi(),
            self.cv());
        format!(
            "digraph G {{\nlabel=\"{}\"\n{}}}",
            stats,
            self.subtasks_to_dot_str(&TaskId::get_root_id()))
    }

    fn subtasks_to_dot_str(&self, root_id: &TaskId) -> String {
        let mut s = String::new();
        let root = self.get(root_id).unwrap();
        let root_str = root.to_dot_str();

        root.child_ids().for_each(|child_id| {
            let child = self.get(&child_id).unwrap();
            s += &format!("\t\"{}\" -> \"{}\"\n", root_str, child.to_dot_str());
            s += &self.subtasks_to_dot_str(&child_id);
        });
        s
    }

    fn subtasks_to_tree_str(&self, root_id: &TaskId, prefix: &str) -> String {
        let mut s = String::new();
        let root = self.get(root_id).unwrap();

        root.child_ids().for_each(|child_id| {
            let child = self.get(&child_id).unwrap();

            match self.next_sibling(&child_id) {
                Ok(_) => {
                    s += &format!("{}├─ {}\n", prefix, child);
                    s += &self.subtasks_to_tree_str(&child_id, &format!("{}│  ", prefix));
                },
                Err(_) => {
                    s += &format!("{}└─ {}\n", prefix, child);
                    s += &self.subtasks_to_tree_str(&child_id, &format!("{}   ", prefix));
                }
            }
        });
        s
    }

    pub fn to_tree_str(&self) -> String {
        let root_id = &TaskId::get_root_id();
        let root = self.get(root_id).unwrap();
        format!(
            "{}\n{}",
            root,
            self.subtasks_to_tree_str(&TaskId::get_root_id(), ""))
    }

    pub fn get(&self, task_id: &TaskId) -> Result<&Task, Error> {
        self.store.get(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }

    pub(crate) fn get_mut(&mut self, task_id: &TaskId) -> Result<&mut Task, Error> {
        self.store.get_mut(task_id)
            .ok_or_else(|| Error::TaskNotFound(task_id.clone()))
    }

    pub(crate) fn insert(&mut self, task_id: TaskId, task: Task) {
        self.store.insert(task_id, task);
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn get_tasks(&self) -> impl Iterator<Item=&Task> {
        self.store
            .values()
            .filter(|task| task.is_leaf())
    }

    pub fn get_todo_tasks(&self) -> impl Iterator<Item=&Task> {
        self.get_tasks()
            .filter(|task| task.status != TaskStatus::Done)
    }

    pub fn get_in_progress_tasks(&self) -> impl Iterator<Item=&Task> {
        self.get_tasks()
            .filter(|task| task.status == TaskStatus::InProgress)
    }

    pub fn get_done_tasks(&self) -> impl Iterator<Item=&Task> {
        self.get_tasks()
            .filter(|task| task.status == TaskStatus::Done)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tasks() {
        let mut tasks = Tasks::new("Project");
        let members = Members::new();

        let root = TaskId::get_root_id();
        let task_id_1 = TaskId::new(vec![1]);
        let task_id_2 = TaskId::new(vec![2]);
        let task_id_3 = TaskId::new(vec![3]);
        let task_id_1_1 = TaskId::new(vec![1, 1]);
        let task_id_2_1 = TaskId::new(vec![2, 1]);
        let task_id_2_2 = TaskId::new(vec![2, 2]);
        let task_id_3_1 = TaskId::new(vec![3, 1]);

        assert!(tasks.add_task(task_id_1.clone(), "Create WSB").is_err());
        assert_eq!(tasks.add_task(root.clone(), "Create WSB"), Ok(&mut Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(tasks.add_task(task_id_1.clone(), "Create Task struct"), Ok(&mut Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        tasks.expand(&[
            ("", "Create CLI tool"),
                ("2", "Create argument parser"),
                ("2", "Create help menu"),
            ("", "Create GUI tool"),
                ("3", "Create plot visualizer")
        ]).unwrap();
        assert_eq!(tasks.get(&task_id_1), Ok(&Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(tasks.get_mut(&task_id_1), Ok(&mut Task::new(TaskId::new(vec![1]), "Create WSB")));

        assert_eq!(tasks.get(&task_id_1_1), Ok(&Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        assert_eq!(tasks.get_mut(&task_id_1_1), Ok(&mut Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        assert_eq!(tasks.set_planned_value(&task_id_1_1, 2.0), Ok(()));
        assert_eq!(tasks.planned_value(), 2.0);
        assert_eq!(tasks.get(&task_id_1_1).unwrap().get_planned_value(), 2.0);
        assert_eq!(tasks.get(&task_id_1).unwrap().get_planned_value(), 2.0);

        assert_eq!(tasks.get(&task_id_2), Ok(&Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        assert_eq!(tasks.get_mut(&task_id_2), Ok(&mut Task::new(TaskId::new(vec![2]), "Create CLI tool")));

        assert_eq!(tasks.get(&task_id_2_1), Ok(&Task::new(TaskId::new(vec![2,1]), "Create argument parser")));
        assert_eq!(tasks.get_mut(&task_id_2_1), Ok(&mut Task::new(TaskId::new(vec![2,1]), "Create argument parser")));
        assert_eq!(tasks.set_planned_value(&task_id_2_1, 7.0), Ok(()));
        assert_eq!(tasks.planned_value(), 9.0);
        assert_eq!(tasks.get(&task_id_2_1).unwrap().get_planned_value(), 7.0);
        assert_eq!(tasks.get(&task_id_2_2).unwrap().get_planned_value(), 0.0);
        assert_eq!(tasks.get(&task_id_2).unwrap().get_planned_value(), 7.0);

        assert_eq!(tasks.get(&task_id_2_2), Ok(&Task::new(TaskId::new(vec![2,2]), "Create help menu")));
        assert_eq!(tasks.get_mut(&task_id_2_2), Ok(&mut Task::new(TaskId::new(vec![2,2]), "Create help menu")));
        assert_eq!(tasks.set_planned_value(&task_id_2_2, 33.0), Ok(()));
        assert_eq!(tasks.planned_value(), 42.0);
        assert_eq!(tasks.get(&task_id_2_1).unwrap().get_planned_value(), 7.0);
        assert_eq!(tasks.get(&task_id_2_2).unwrap().get_planned_value(), 33.0);
        assert_eq!(tasks.get(&task_id_2).unwrap().get_planned_value(), 40.0);

        assert_eq!(tasks.get(&task_id_3), Ok(&Task::new(TaskId::new(vec![3]), "Create GUI tool")));
        assert_eq!(tasks.get_mut(&task_id_3), Ok(&mut Task::new(TaskId::new(vec![3]), "Create GUI tool")));

        assert_eq!(tasks.get(&task_id_3_1), Ok(&Task::new(TaskId::new(vec![3,1]), "Create plot visualizer")));
        assert_eq!(tasks.get_mut(&task_id_3_1), Ok(&mut Task::new(TaskId::new(vec![3,1]), "Create plot visualizer")));
        assert_eq!(tasks.set_planned_value(&task_id_3_1, 20.0), Ok(()));
        assert_eq!(tasks.planned_value(), 62.0);
        assert_eq!(tasks.get(&task_id_3_1).unwrap().get_planned_value(), 20.0);
        assert_eq!(tasks.get(&task_id_3).unwrap().get_planned_value(), 20.0);
        assert_eq!(tasks.remove(&task_id_2_1, &members), Ok(Task::new(TaskId::new(vec![2,1]), "Create argument parser")));

        assert_eq!(tasks.planned_value(), 55.0);
        assert_eq!(tasks.get(&task_id_2_1), Ok(&Task::new(TaskId::new(vec![2, 1]), "Create help menu")));
        assert_eq!(tasks.get(&task_id_2), Ok(&Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        assert_eq!(tasks.get(&task_id_2).unwrap().get_planned_value(), 33.0);

        assert_eq!(tasks.remove(&task_id_2, &members), Err(Error::TrunkCannotBeRemoved(task_id_2.clone())));
        assert_eq!(tasks.planned_value(), 55.0);
        assert_eq!(tasks.remove(&task_id_2_1, &members), Ok(Task::new(TaskId::new(vec![2,1]), "Create help menu")));
        assert_eq!(tasks.planned_value(), 22.0);
        assert_eq!(tasks.get(&task_id_2).unwrap().get_planned_value(), 0.0);
        assert_eq!(tasks.remove(&task_id_2, &members), Ok(Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        assert_eq!(tasks.planned_value(), 22.0);

        assert_eq!(tasks.get(&task_id_1), Ok(&Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(tasks.get_mut(&task_id_1), Ok(&mut Task::new(TaskId::new(vec![1]), "Create WSB")));

        assert_eq!(tasks.get(&task_id_1_1), Ok(&Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        assert_eq!(tasks.get_mut(&task_id_1_1), Ok(&mut Task::new(TaskId::new(vec![1,1]), "Create Task struct")));

        assert_eq!(tasks.get(&task_id_2), Ok(&Task::new(TaskId::new(vec![2]), "Create GUI tool")));
        assert_eq!(tasks.get_mut(&task_id_2), Ok(&mut Task::new(TaskId::new(vec![2]), "Create GUI tool")));

        assert_eq!(tasks.get(&task_id_2_1), Ok(&Task::new(TaskId::new(vec![2,1]), "Create plot visualizer")));
        assert_eq!(tasks.get_mut(&task_id_2_1), Ok(&mut Task::new(TaskId::new(vec![2,1]), "Create plot visualizer")));
    }
}
