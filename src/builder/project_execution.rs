use crate::{project::Project, task::Task, error::Error};

use super::wsb_execution::WSBExecution;

#[derive(Debug)]
enum ProjectAction {
    Save,
    SaveTo(String),
    AddMember(String),
    RunWSBBuilder(WSBExecution),
}

pub enum Return {
    Task(Task),
    VisualizationDot(Option<String>, String),
    VisualizationTree(Option<String>, String)
}

pub struct ProjectExecution {
    actions: Vec<ProjectAction>,
    project: Project
}

impl ProjectExecution {

    pub fn new(name: &str) -> Self {
        Self {
            actions: Vec::new(),
            project: Project::new(name),
        }
    }

    pub fn load(name: &str) -> Result<Self, Error> {
        Ok(Self {
            actions: Vec::new(),
            project: Project::load(&name)?,
        })
    }

    pub fn save(mut self) -> Self {
        self.actions.push(ProjectAction::Save);
        self
    }

    pub fn add_member(mut self, name: &str) -> Self {
        self.actions.push(ProjectAction::AddMember(name.to_string()));
        self
    }

    pub fn wsb<F: FnMut(&mut WSBExecution)>(mut self, mut func: F) -> Self {
        let mut wsb_execution = WSBExecution::new();
        func(&mut wsb_execution);
        self.actions.push(ProjectAction::RunWSBBuilder(wsb_execution));
        self
    }

    pub fn run(mut self) -> Result<Vec<Return>, Error> {
        let mut results : Vec<Return> = Vec::new();
        self.actions
            .into_iter()
            .try_for_each(|action| -> Result<(), Error> {
                match action {
                    ProjectAction::Save => { self.project.save(); },
                    ProjectAction::SaveTo(filename) => { self.project.save_to(&filename); },
                    ProjectAction::AddMember(name) => { self.project.add_member(&name); },
                    ProjectAction::RunWSBBuilder(wsb_execution) => { results.append(&mut wsb_execution.run(&mut self.project)?); },
                }
                Ok(())
            });
        Ok(results)
    }
}
