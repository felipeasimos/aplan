use std::collections::HashMap;
use std::io::Write;

use crate::task::{Task, TaskStatus};
use crate::task::task_id::TaskId;

#[derive(Debug)]
pub struct WSB;

impl WSB {
    pub fn new(name: &str, map: &mut HashMap<TaskId, Task>) -> Self {
        let root_id = TaskId::get_root_id();
        let root_task = Task::new(root_id.clone(), name);
        map.insert(root_id, root_task);
        Self {}
    }

    pub fn name<'a>(&'a self, tree: &'a HashMap<TaskId, Task>) -> &str {
        tree.get(&TaskId::get_root_id()).unwrap().name()
    }

    pub fn planned_value(&self, tree: &HashMap<TaskId, Task>) -> f64 {
        tree.get(&TaskId::get_root_id()).unwrap().get_planned_value()
    }

    pub fn actual_cost(&self, tree: &HashMap<TaskId, Task>) -> f64 {
        tree.get(&TaskId::get_root_id()).unwrap().get_actual_cost()
    }

    pub fn completion_percentage(&self, tree: &HashMap<TaskId, Task>) -> f64 {
        self.done_tasks(tree).len() as f64 / tree.len() as f64
    }

    pub fn earned_value(&self, tree: &HashMap<TaskId, Task>) -> f64 {
        self.planned_value(tree) * self.completion_percentage(tree)
    }

    pub fn spi(&self, tree: &HashMap<TaskId, Task>) -> f64 {
        self.earned_value(tree) / self.planned_value(tree)
    }

    pub fn sv(&self, tree: &HashMap<TaskId, Task>) -> f64 {
        self.earned_value(tree) - self.planned_value(tree)
    }

    pub fn cpi(&self, tree: &HashMap<TaskId, Task>) -> f64 {
        self.earned_value(tree) / self.actual_cost(tree)
    }

    pub fn cv(&self, tree: &HashMap<TaskId, Task>) -> f64 {
        self.earned_value(tree) - self.actual_cost(tree)
    }

    pub fn get_task<'a>(&'a self, task_id: &TaskId, tree: &'a HashMap<TaskId, Task>) -> Option<&Task> {
        tree.get(&task_id)
    }

    pub fn get_task_mut<'a>(&'a mut self, task_id: &TaskId, tree: &'a mut HashMap<TaskId, Task>) -> Option<&mut Task> {
        tree.get_mut(&task_id)
    }

    pub fn add_task<'a>(&'a mut self, mut parent_task_id: TaskId, name: &str, tree: &'a mut HashMap<TaskId, Task>) -> Option<&mut Task> {
        // get parent
        let parent_task = tree.get_mut(&parent_task_id)?;

        // increase number of children
        parent_task.num_child += 1;

        // get new task id
        parent_task_id.as_vec_mut().push(parent_task.num_child);
        let task_id = parent_task_id;

        // create task
        let task = Task::new(task_id.clone(), name);

        // add task to task map
        tree.insert(task_id.clone(), task);

        // since new tasks are always not done, all parents must be not done too
        self.apply_along_path(&task_id, |task| {
            task.status = TaskStatus::InProgress;
        }, tree);

        tree.get_mut(&task_id)
    }

    pub fn expand<const N: usize>(&mut self, arr: &[(&str, &str); N], tree: &mut HashMap<TaskId, Task>) -> Option<&mut Self> {
        for (parent_id, task_name) in arr {
            self.add_task(TaskId::parse(parent_id).unwrap(), task_name, tree)?;
        }
        Some(self)
    }

    fn apply_along_path<F: Fn(&mut Task)>(&mut self, id: &TaskId, func: F, tree: &mut HashMap<TaskId, Task>) -> Option<()> {
        id
            .path()
            .iter()
            .try_for_each(|id| {
                let child = tree.get_mut(&id)?;
                func(child);
                Some(())
            })
    }

    fn subtract_id(&mut self, child_id: &TaskId, layer_idx: usize, tree: &mut HashMap<TaskId, Task>) {
        let num_child = tree.get(child_id).unwrap().num_child;
        let old_task_id = child_id.clone();
        let mut new_task_id = child_id.clone();
        new_task_id.as_vec_mut()[layer_idx] -= 1;
        let mut task = tree.remove(&old_task_id).unwrap();
        task.id = new_task_id.clone();
        tree.insert(
            new_task_id,
            task
        );

        child_id.child_ids(num_child).iter().for_each(|node_id| {
            self.subtract_id(node_id, layer_idx, tree)
        })
    }

    pub fn remove(&mut self, task_id: &TaskId, tree: &mut HashMap<TaskId, Task>) -> Option<Task> {
        // don't remove if this is a trunk node
        let mut task_id = task_id.clone();
        if tree.get(&task_id)?.num_child > 0 {
            return None;
        }

        self.remove_task_stats_from_tree(&task_id, tree);
        let parent_id = task_id.parent()?;
        let parent_childs = {
            let mut parent = tree.get_mut(&parent_id)?;
            let ids = parent.child_ids();
            parent.num_child -= 1;
            ids
        };

        let layer_idx = task_id.as_vec().len() - 1;
        let child_idx = (*task_id.as_vec().last()? as usize) - 1;

        let task = tree.remove(&task_id)?;

        // change id of child that comes after id node
        parent_childs.iter().enumerate().for_each(|(index, child_id)| {
            if child_idx < index {
                self.subtract_id(child_id, layer_idx, tree);
            }
        });

        // remove last id child from the parent
        task_id.as_vec_mut()[layer_idx] = parent_childs.len() as u32;
        tree.remove(&task_id);

        Some(task)
    }

    fn remove_task_stats_from_tree(&mut self, task_id: &TaskId, tree: &mut HashMap<TaskId, Task>) {

        self.set_actual_cost(&task_id, 0.0, tree);
        self.set_planned_value(&task_id, 0.0, tree);
    }

    fn children_are_done(&self, task_id: &TaskId, tree: &HashMap<TaskId, Task>) -> bool {
        tree.get(task_id).unwrap()
            .child_ids()
            .iter()
            .find(|id| tree.get(id).unwrap().status != TaskStatus::Done)
            .is_none()
    }

    pub fn set_actual_cost(&mut self, task_id: &TaskId, actual_cost: f64, tree: &mut HashMap<TaskId, Task>) -> Option<()> {
        let parent_id = task_id.parent()?;
        {
            let mut task = tree.get_mut(&task_id)?;
            if task.is_trunk() {
                return None;
            }
            let old_actual_cost = task.actual_cost;
            task.actual_cost = actual_cost;
            let diff = actual_cost - old_actual_cost;

                self.apply_along_path(&parent_id, |mut task| {
                    task.actual_cost += diff;
                }, tree);
        }

        task_id
            .clone()
            .path()
            .iter()
            .rev()
            .try_for_each(|id| {
                if self.children_are_done(&id, tree) {
                    tree.get_mut(&id)?.status = TaskStatus::Done;
                }
                Some(())
            });
        Some(())
    }

    pub fn set_planned_value(&mut self, task_id: &TaskId, planned_value: f64, tree: &mut HashMap<TaskId, Task>) -> Option<()> {
        let parent_id = task_id.parent()?;
        let mut task = tree.get_mut(&task_id)?;
        // can't set actual cost of trunk node
        if task.is_trunk() {
            return None;
        }
        let old_planned_value = task.planned_value;
        task.planned_value = planned_value;
        let diff = planned_value - old_planned_value;

        self.apply_along_path(&parent_id, |mut task| {
            task.planned_value += diff;
        }, tree)
    }

    fn subtree_to_dot_str(&self, root_id: &TaskId, tree: &HashMap<TaskId, Task>) -> String {
        let mut s = String::new();
        let root = tree.get(root_id).unwrap();
        let root_str = root.to_string();

        root.child_ids().iter().for_each(|child_id| {
            let child = tree.get(child_id).unwrap();
            s += &format!("\t\"{}\" -> \"{}\"\n", root_str, child.to_string());
            s += &self.subtree_to_dot_str(child_id, tree);
        });
        s
    }

    pub fn to_dot_str(&self, tree: &HashMap<TaskId, Task>) -> String {
        "digraph G {\n".to_string() +
            &self.subtree_to_dot_str(&TaskId::get_root_id(), tree) +
            &"}".to_string()
    }

    pub fn tasks<'a>(&'a self, tree: &'a HashMap<TaskId, Task>) -> Vec<&Task> {
        tree
            .values()
            .filter(|task| task.is_leaf())
            .collect::<Vec<&Task>>()
    }

    pub fn todo_tasks<'a>(&'a self, tree: &'a HashMap<TaskId, Task>) -> Vec<&Task> {
        tree
            .values()
            .filter(|task| task.is_leaf() && task.status != TaskStatus::Done)
            .collect::<Vec<&Task>>()
    }

    pub fn in_progress_tasks<'a>(&'a self, tree: &'a HashMap<TaskId, Task>) -> Vec<&Task> {
        tree
            .values()
            .filter(|task| task.is_leaf() && task.status == TaskStatus::InProgress)
            .collect::<Vec<&Task>>()
    }

    pub fn done_tasks<'a>(&'a self, tree: &'a HashMap<TaskId, Task>) -> Vec<&Task> {
        tree
            .values()
            .filter(|task| task.is_leaf() && task.status == TaskStatus::Done)
            .collect::<Vec<&Task>>()
    }

    pub fn to_dot_file(&self, filename: &str, tree: &mut HashMap<TaskId, Task>) {
        write!(std::fs::File::create(filename).unwrap(), "{}", self.to_dot_str(tree)).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tasks() {
        let mut tasks = HashMap::new();
        let map = &mut tasks;
        let mut wsb = WSB::new("Project", map);

        let root = TaskId::get_root_id();
        let task_id_1 = TaskId::new(vec![1]);
        let task_id_2 = TaskId::new(vec![2]);
        let task_id_3 = TaskId::new(vec![3]);
        let task_id_1_1 = TaskId::new(vec![1, 1]);
        let task_id_2_1 = TaskId::new(vec![2, 1]);
        let task_id_2_2 = TaskId::new(vec![2, 2]);
        let task_id_3_1 = TaskId::new(vec![3, 1]);

        assert!(wsb.add_task(task_id_1.clone(), "Create WSB", map).is_none());
        assert_eq!(wsb.add_task(root.clone(), "Create WSB", map), Some(&mut Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(wsb.add_task(task_id_1.clone(), "Create Task struct", map), Some(&mut Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        wsb.expand(&[
            ("", "Create CLI tool"),
                ("2", "Create argument parser"),
                ("2", "Create help menu"),
            ("", "Create GUI tool"),
                ("3", "Create plot visualizer")
        ], map);
        assert_eq!(wsb.get_task(&task_id_1, map), Some(&Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(wsb.get_task_mut(&task_id_1, map), Some(&mut Task::new(TaskId::new(vec![1]), "Create WSB")));

        assert_eq!(wsb.get_task(&task_id_1_1, map), Some(&Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        assert_eq!(wsb.get_task_mut(&task_id_1_1, map), Some(&mut Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        assert_eq!(wsb.set_planned_value(&task_id_1_1, 2.0, map), Some(()));
        assert_eq!(wsb.planned_value(map), 2.0);
        assert_eq!(wsb.get_task(&task_id_1_1, map).unwrap().get_planned_value(), 2.0);
        assert_eq!(wsb.get_task(&task_id_1, map).unwrap().get_planned_value(), 2.0);

        assert_eq!(wsb.get_task(&task_id_2, map), Some(&Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        assert_eq!(wsb.get_task_mut(&task_id_2, map), Some(&mut Task::new(TaskId::new(vec![2]), "Create CLI tool")));

        assert_eq!(wsb.get_task(&task_id_2_1, map), Some(&Task::new(TaskId::new(vec![2,1]), "Create argument parser")));
        assert_eq!(wsb.get_task_mut(&task_id_2_1, map), Some(&mut Task::new(TaskId::new(vec![2,1]), "Create argument parser")));
        assert_eq!(wsb.set_planned_value(&task_id_2_1, 7.0, map), Some(()));
        assert_eq!(wsb.planned_value(map), 9.0);
        assert_eq!(wsb.get_task(&task_id_2_1, map).unwrap().get_planned_value(), 7.0);
        assert_eq!(wsb.get_task(&task_id_2_2, map).unwrap().get_planned_value(), 0.0);
        assert_eq!(wsb.get_task(&task_id_2, map).unwrap().get_planned_value(), 7.0);

        assert_eq!(wsb.get_task(&task_id_2_2, map), Some(&Task::new(TaskId::new(vec![2,2]), "Create help menu")));
        assert_eq!(wsb.get_task_mut(&task_id_2_2, map), Some(&mut Task::new(TaskId::new(vec![2,2]), "Create help menu")));
        assert_eq!(wsb.set_planned_value(&task_id_2_2, 33.0, map), Some(()));
        assert_eq!(wsb.planned_value(map), 42.0);
        assert_eq!(wsb.get_task(&task_id_2_1, map).unwrap().get_planned_value(), 7.0);
        assert_eq!(wsb.get_task(&task_id_2_2, map).unwrap().get_planned_value(), 33.0);
        assert_eq!(wsb.get_task(&task_id_2, map).unwrap().get_planned_value(), 40.0);

        assert_eq!(wsb.get_task(&task_id_3, map), Some(&Task::new(TaskId::new(vec![3]), "Create GUI tool")));
        assert_eq!(wsb.get_task_mut(&task_id_3, map), Some(&mut Task::new(TaskId::new(vec![3]), "Create GUI tool")));

        assert_eq!(wsb.get_task(&task_id_3_1, map), Some(&Task::new(TaskId::new(vec![3,1]), "Create plot visualizer")));
        assert_eq!(wsb.get_task_mut(&task_id_3_1, map), Some(&mut Task::new(TaskId::new(vec![3,1]), "Create plot visualizer")));
        assert_eq!(wsb.set_planned_value(&task_id_3_1, 20.0, map), Some(()));
        assert_eq!(wsb.planned_value(map), 62.0);
        assert_eq!(wsb.get_task(&task_id_3_1, map).unwrap().get_planned_value(), 20.0);
        assert_eq!(wsb.get_task(&task_id_3, map).unwrap().get_planned_value(), 20.0);
        assert_eq!(wsb.remove(&task_id_2_1, map), Some(Task::new(TaskId::new(vec![2,1]), "Create argument parser")));

        assert_eq!(wsb.planned_value(map), 55.0);
        assert_eq!(wsb.get_task(&task_id_2_1, map), Some(&Task::new(TaskId::new(vec![2, 1]), "Create help menu")));
        assert_eq!(wsb.get_task(&task_id_2, map), Some(&Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        assert_eq!(wsb.get_task(&task_id_2, map).unwrap().get_planned_value(), 33.0);

        assert_eq!(wsb.remove(&task_id_2, map), None);
        assert_eq!(wsb.planned_value(map), 55.0);
        assert_eq!(wsb.remove(&task_id_2_1, map), Some(Task::new(TaskId::new(vec![2,1]), "Create help menu")));
        assert_eq!(wsb.planned_value(map), 22.0);
        assert_eq!(wsb.get_task(&task_id_2, map).unwrap().get_planned_value(), 0.0);
        assert_eq!(wsb.remove(&task_id_2, map), Some(Task::new(TaskId::new(vec![2]), "Create CLI tool")));
        assert_eq!(wsb.planned_value(map), 22.0);

        assert_eq!(wsb.get_task(&task_id_1, map), Some(&Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(wsb.get_task_mut(&task_id_1, map), Some(&mut Task::new(TaskId::new(vec![1]), "Create WSB")));

        assert_eq!(wsb.get_task(&task_id_1_1, map), Some(&Task::new(TaskId::new(vec![1,1]), "Create Task struct")));
        assert_eq!(wsb.get_task_mut(&task_id_1_1, map), Some(&mut Task::new(TaskId::new(vec![1,1]), "Create Task struct")));

        assert_eq!(wsb.get_task(&task_id_2, map), Some(&Task::new(TaskId::new(vec![2]), "Create GUI tool")));
        assert_eq!(wsb.get_task_mut(&task_id_2, map), Some(&mut Task::new(TaskId::new(vec![2]), "Create GUI tool")));

        assert_eq!(wsb.get_task(&task_id_2_1, map), Some(&Task::new(TaskId::new(vec![2,1]), "Create plot visualizer")));
        assert_eq!(wsb.get_task_mut(&task_id_2_1, map), Some(&mut Task::new(TaskId::new(vec![2,1]), "Create plot visualizer")));
    }
}
