use crate::{task::task_id::TaskId, project::Project, error::Error, prelude::Task};

use super::project_execution::Return;

#[derive(Debug, Clone)]
enum WSBAction {
    Dot(Option<String>),
    Tree(Option<String>),
    Add(TaskId, String),
    Remove(TaskId),
    PlannedValue(TaskId, f64),
    Done(TaskId, f64),
    GetTask(TaskId),
    Todo(Option<String>)
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

    pub fn get_todo_tasks(&mut self, name: Option<String>) -> &mut Self {
        self.actions.push(WSBAction::Todo(name));
        self
    } 

    pub(crate) fn run(self, project: &mut Project) -> Result<Vec<Return>, Error> {
        Ok(self.actions
            .into_iter()
            .map(|action| -> Result<Option<Return>, Error> {
                Ok(match &action {
                    WSBAction::Dot(filename) => Some(Return::Dot(filename.clone(), project.wsb.to_dot_str(&mut project.tasks))),
                    WSBAction::Tree(filename) => Some(Return::Tree(filename.clone(), project.wsb.to_tree_str(&mut project.tasks))),
                    WSBAction::Add(parent_id, name) => { project.wsb.add_task(parent_id.clone(), &name, &mut project.tasks)?; None },
                    WSBAction::Remove(id) => { project.wsb.remove(&id, &mut project.tasks)?; None },
                    WSBAction::Done(id, cost) => { project.wsb.set_actual_cost(&id, *cost, &mut project.tasks)?; None },
                    WSBAction::PlannedValue(id, value) => { project.wsb.set_planned_value(&id, *value, &mut project.tasks)?; None },
                    WSBAction::GetTask(id) => Some(Return::Task(project.tasks.get(&id)?.clone())),
                    WSBAction::Todo(Some(name)) => Some(Return::Tasks(project.wsb.todo_tasks(&mut project.tasks).filter(|t| t.has_member(name)).cloned().collect::<Vec<Task>>())),
                    WSBAction::Todo(None) => Some(Return::Tasks(project.wsb.todo_tasks(&mut project.tasks).cloned().collect::<Vec<Task>>()))
                })
            })
            .collect::<Result<Vec<Option<Return>>, Error>>()?
            .into_iter()
            .filter_map(|res| res)
            .collect::<Vec<Return>>())
    }
}
