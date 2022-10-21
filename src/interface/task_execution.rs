use crate::{task::task_id::TaskId, project::Project, error::Error};

#[derive(Debug)]
pub struct TaskExecution<'a> {
    project: &'a mut Project
}

impl<'a> TaskExecution<'a> {

    pub(crate) fn new(project: &'a mut Project) -> TaskExecution {
        Self {
            project
        }
    }

    pub fn add(&mut self, id: TaskId, name: &str) -> Result<&mut Self, Error> {
        self.project.tasks.add_task(id, name)?;
        Ok(self)
    }

    pub fn remove(&mut self, id: &TaskId) -> Result<&mut Self, Error> {
        self.project.tasks.remove(id, &self.project.members)?;
        Ok(self)
    }

    pub fn expand<const N: usize>(&mut self, arr: &[(&str, &str); N]) -> Result<&mut Self, Error> {
        self.project.tasks.expand(arr)?;
        Ok(self)
    }

    pub fn done(&mut self, id: &TaskId, cost: f64) -> Result<&mut Self, Error> {
        self.project.tasks.set_actual_cost(id, cost)?;
        Ok(self)
    }

    pub fn planned_value(&mut self, id: &TaskId, planned_value: f64) -> Result<&mut Self, Error> {
        self.project.tasks.set_planned_value(id, planned_value)?;
        Ok(self)
    }
}
