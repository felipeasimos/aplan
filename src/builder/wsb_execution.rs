use crate::{task::task_id::TaskId, project::Project, error::Error, util};

use super::project_execution::Return;

#[derive(Debug, Clone)]
enum WSBAction {
    Dot(Option<String>),
    Tree(Option<String>),
    Add(TaskId, String),
    Remove(TaskId),
    PlannedValue(TaskId, f64),
    Done(TaskId, f64),
    GetTask(TaskId)
}

#[derive(Debug, Clone)]
pub struct WSBExecution {

    actions: Vec<WSBAction>
}

impl WSBExecution {

    pub(crate) fn new() -> WSBExecution {
        Self {
            actions: Vec::new()
        }
    }

    pub fn dot(&mut self, filename: Option<&str>) -> &mut Self {
        self.actions.push(WSBAction::Dot(filename.map(|s| s.to_string())));
        self
    }

    pub fn tree(&mut self, filename: Option<&str>) -> &mut Self {
        self.actions.push(WSBAction::Tree(filename.map(|s| s.to_string())));
        self
    }

    pub fn add(&mut self, id: &TaskId, name: &str) -> &mut Self {
        self.actions.push(WSBAction::Add(id.clone(), name.to_string()));
        self
    }

    pub fn remove(&mut self, id: &TaskId) -> &mut Self {
        self.actions.push(WSBAction::Remove(id.clone()));
        self
    }

    pub fn expand<const N: usize>(&mut self, arr: &[(&str, &str); N]) -> &mut Self {
        for (parent_id, task_name) in arr {
            self.add(&TaskId::parse(parent_id).unwrap(), task_name);
        }
        self
    }

    pub fn done(&mut self, id: &TaskId, cost: f64) -> &mut Self {
        self.actions.push(WSBAction::Done(id.clone(), cost));
        self
    }

    pub fn planned_value(&mut self, id: &TaskId, planned_value: f64) -> &mut Self {
        self.actions.push(WSBAction::PlannedValue(id.clone(), planned_value));
        self
    }

    pub fn get_task(&mut self, id: &TaskId) -> &mut Self {
        self.actions.push(WSBAction::GetTask(id.clone()));
        self
    }

    pub(crate) fn run(self, project: &mut Project) -> Result<Vec<Return>, Error> {
        let mut results = Vec::new();
        self.actions
            .into_iter()
            .try_for_each(|action| -> Result<(), Error> {
                match &action {
                    WSBAction::Dot(filename) => { util::to_file(filename.as_deref(), project.wsb.to_dot_str(&mut project.tasks)); },
                    WSBAction::Tree(filename) => { util::to_file(filename.as_deref(), project.wsb.to_tree_str(&mut project.tasks)); },
                    WSBAction::Add(parent_id, name) => { project.wsb.add_task(parent_id.clone(), &name, &mut project.tasks); },
                    WSBAction::Remove(id) => { project.wsb.remove(&id, &mut project.tasks); },
                    WSBAction::Done(id, cost) => { project.wsb.set_actual_cost(&id, *cost, &mut project.tasks); },
                    WSBAction::PlannedValue(id, value) => { project.wsb.set_planned_value(&id, *value, &mut project.tasks); },
                    WSBAction::GetTask(id) => { results.push(Return::Task(project.wsb.get_task(&id, &mut project.tasks)?.clone())); },
                }
                Ok(())
            });
        Ok(results)
    }
}
