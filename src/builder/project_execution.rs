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
    Dot(Option<String>, String),
    Tree(Option<String>, String),
    MembersList(Vec<Member>),
    Member(Member)
}

pub type Aplan = ProjectExecution;

pub struct ProjectExecution {
    actions: Vec<ProjectAction>,
    project: Project
}

impl ProjectExecution {

    pub fn from(project: Project) -> Self {
        Self {
            actions: Vec::new(),
            project
        }
    }

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

    pub fn run<F: FnMut(&Vec<Return>) -> Result<(), Error>>(mut self, mut func: F) -> Result<Project, Error> {
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
        func(&results)
            .map(|_| self.project)
    }
}
