use crate::{project::Project, task::Task, error::Error, member::Member};

use super::{wsb_execution::WSBExecution, member_execution::MemberExecution};

#[derive(Debug)]
enum ProjectAction {
    Save,
    SaveTo(String),
    RunWSB(WSBExecution),
    RunMember(MemberExecution)
}

pub enum Return {
    Task(Task),
    VisualizationDot(Option<String>, String),
    VisualizationTree(Option<String>, String),
    MembersList(Vec<Member>)
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

    pub fn save_to(mut self, filename: &str) -> Self {
        self.actions.push(ProjectAction::SaveTo(filename.to_string()));
        self
    }

    pub fn wsb<F: FnMut(&mut WSBExecution)>(mut self, mut func: F) -> Self {
        let mut wsb_execution = WSBExecution::new();
        func(&mut wsb_execution);
        self.actions.push(ProjectAction::RunWSB(wsb_execution));
        self
    }

    pub fn member<F: FnMut(&mut MemberExecution)>(mut self, mut func: F) -> Self {
        let mut member_execution = MemberExecution::new();
        func(&mut member_execution);
        self.actions.push(ProjectAction::RunMember(member_execution));
        self
    }

    pub fn run(mut self) -> Result<Vec<Return>, Error> {
        let mut results : Vec<Return> = Vec::new();
        self.actions
            .into_iter()
            .try_for_each(|action| -> Result<(), Error> {
                match action {
                    ProjectAction::Save => { self.project.save()?; },
                    ProjectAction::SaveTo(filename) => { self.project.save_to(&filename)?; },
                    ProjectAction::RunWSB(wsb_execution) => { results.append(&mut wsb_execution.run(&mut self.project)?); },
                    ProjectAction::RunMember(member_execution) => { results.append(&mut member_execution.run(&mut self.project)?); },
                }
                Ok(())
            })?;
        Ok(results)
    }
}
