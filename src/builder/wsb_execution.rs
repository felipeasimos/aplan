use crate::{task::{task_id::TaskId, Task}, subsystem::wsb::WSB, project::Project};

use super::project_execution::Return;

#[derive(Debug, Clone)]
enum WSBAction {
    Show(String),
    Add(TaskId, String),
    Remove(TaskId),
    Value(TaskId, f64),
    Done(TaskId, f64),
    GetTask(TaskId)
}

#[derive(Debug)]
pub struct WSBExecution {

    actions: Vec<WSBAction>
}

impl WSBExecution {

    pub(crate) fn new() -> WSBExecution {
        Self {
            actions: Vec::new()
        }
    }

    pub fn show(&mut self, filename: &str) -> &mut Self {
        self.actions.push(WSBAction::Show(filename.to_string()));
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

    pub fn value(&mut self, id: &TaskId, planned_value: f64) -> &mut Self {
        self.actions.push(WSBAction::Value(id.clone(), planned_value));
        self
    }

    pub fn get_task(&mut self, id: &TaskId) -> &mut Self {
        self.actions.push(WSBAction::GetTask(id.clone()));
        self
    }

    pub(crate) fn run(self, project: &mut Project) -> Vec<Return> {
        let mut results = Vec::new();
        self.actions
            .into_iter()
            .for_each(|action| match &action {
                WSBAction::Show(filename) => { project.wsb.to_dot_file(&filename, &mut project.tasks); },
                WSBAction::Add(parent_id, name) => { project.wsb.add_task(parent_id.clone(), &name, &mut project.tasks).unwrap(); },
                WSBAction::Remove(id) => { project.wsb.remove(&id, &mut project.tasks).unwrap(); },
                WSBAction::Done(id, cost) => { project.wsb.set_actual_cost(&id, *cost, &mut project.tasks).unwrap(); },
                WSBAction::Value(id, value) => { project.wsb.set_planned_value(&id, *value, &mut project.tasks).unwrap(); },
                WSBAction::GetTask(id) => { results.push(Return::Task(project.wsb.get_task(&id, &mut project.tasks).unwrap().clone())); },
            });
        results
    }
}