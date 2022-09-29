use crate::{project::Project, task::Task};

use super::wsb_execution::WSBExecution;

#[derive(Debug)]
enum ProjectAction {
    Save,
    SaveTo(String),
    ReturnProject,
    RunWSBBuilder(WSBExecution),
}

pub enum Return {
    Task(Task),
    Project(Project)
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

    pub fn load(name: &str) -> Option<Self> {
        Some(Self {
            actions: Vec::new(),
            project: Project::load(&name).unwrap(),
        })
    }

    pub fn save(mut self) -> Self {
        self.actions.push(ProjectAction::Save);
        self
    }

    pub fn wsb<F: FnMut(&mut WSBExecution)>(mut self, mut func: F) -> Self {
        let mut wsb_execution = WSBExecution::new();
        func(&mut wsb_execution);
        self.actions.push(ProjectAction::RunWSBBuilder(wsb_execution));
        self
    }

    pub fn return_project(mut self) -> Self {
        self.actions.push(ProjectAction::ReturnProject);
        self
    }

    pub fn run(mut self) -> Vec<Return> {
        let mut results : Vec<Return> = Vec::new();
        self.actions
            .into_iter()
            .for_each(|action| match action {
                ProjectAction::Save => { self.project.save().unwrap(); },
                ProjectAction::SaveTo(filename) => { self.project.save_to(&filename).unwrap(); },
                ProjectAction::ReturnProject => { results.push(Return::Project(self.project.clone())); },
                ProjectAction::RunWSBBuilder(wsb_execution) => { results.append(&mut wsb_execution.run(&mut self.project)); },
            });
        results
    }
}
