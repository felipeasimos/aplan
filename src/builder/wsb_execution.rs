use crate::{task::task_id::TaskId, project::Project, error::Error, prelude::Task};

#[derive(Debug)]
pub struct WSBExecution<'a> {
    project: &'a mut Project
}

impl<'a> WSBExecution<'a> {

    pub(crate) fn new(project: &'a mut Project) -> WSBExecution {
        Self {
            project
        }
    }

    pub fn get_task(&self, id: &TaskId) -> Result<&Task, Error> {
        self.project.tasks.get(id)
    }

    pub fn get_tasks(&self) -> impl Iterator<Item=&Task> {
        self.project.tasks.leaf_tasks()
    }

    pub fn get_done_tasks(&self) -> impl Iterator<Item=&Task> {
        self.project.tasks.done_tasks()
    }

    pub fn get_in_progress_tasks(&self) -> impl Iterator<Item=&Task> {
        self.project.tasks.in_progress_tasks()
    }

    pub fn get_todo_tasks(&self) -> impl Iterator<Item=&Task> {
        self.project.tasks.todo_tasks()
    }

    pub fn add(&mut self, id: TaskId, name: &str) -> Result<&mut Self, Error> {
        self.project.wsb.add_task(id, name, &mut self.project.tasks)?;
        Ok(self)
    }

    pub fn remove(&mut self, id: &TaskId) -> Result<&mut Self, Error> {
        self.project.wsb.remove(id, &mut self.project.tasks)?;
        Ok(self)
    }

    pub fn expand<const N: usize>(&mut self, arr: &[(&str, &str); N]) -> Result<&mut Self, Error> {
        for (parent_id, task_name) in arr {
            self.add(TaskId::parse(parent_id)?, task_name)?;
        }
        Ok(self)
    }

    pub fn done(&mut self, id: &TaskId, cost: f64) -> Result<&mut Self, Error> {
        self.project.wsb.set_actual_cost(id, cost, &mut self.project.tasks)?;
        Ok(self)
    }

    pub fn planned_value(&mut self, id: &TaskId, planned_value: f64) -> Result<&mut Self, Error> {
        self.project.wsb.set_planned_value(id, planned_value, &mut self.project.tasks)?;
        Ok(self)
    }

    pub fn dot(&self) -> String {
        self.project.wsb.to_dot_str(&self.project.tasks)
    }

    pub fn tree(&self) -> String {
        self.project.wsb.to_tree_str(&self.project.tasks)
    }
}
