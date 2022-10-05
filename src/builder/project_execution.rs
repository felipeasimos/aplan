use crate::{project::Project, task::{Task, task_id::TaskId}, error::Error};

use super::wsb_execution::WSBExecution;

#[derive(Debug)]
enum ProjectAction {
    Save,
    SaveTo(String),
    AddMember(String),
    RemoveMember(String),
    AssignTaskToMember(TaskId, String),
    RemoveMemberFromTask(TaskId, String),
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

    pub fn save_to(mut self, filename: &str) -> Self {
        self.actions.push(ProjectAction::SaveTo(filename.to_string()));
        self
    }

    pub fn add_member(mut self, name: &str) -> Self {
        self.actions.push(ProjectAction::AddMember(name.to_string()));
        self
    }

    pub fn remove_member(mut self, name: &str) -> Self {
        self.actions.push(ProjectAction::RemoveMember(name.to_string()));
        self
    }

    pub fn assign_task_to_member(mut self, id: TaskId, name: &str) -> Self {
        self.actions.push(ProjectAction::AssignTaskToMember(id, name.to_string()));
        self
    }

    pub fn remove_member_from_task(mut self, id: TaskId, name: &str) -> Self {
        self.actions.push(ProjectAction::RemoveMemberFromTask(id, name.to_string()));
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
                    ProjectAction::Save => { self.project.save()?; },
                    ProjectAction::SaveTo(filename) => { self.project.save_to(&filename)?; },
                    ProjectAction::AddMember(name) => { self.project.add_member(&name); },
                    ProjectAction::RemoveMember(name) => { self.project.remove_member(&name)?; },
                    ProjectAction::AssignTaskToMember(task_id, name) => { self.project.assign_task_to_member(task_id, &name)?; },
                    ProjectAction::RemoveMemberFromTask(task_id, name) => { self.project.remove_member_from_task(&task_id, &name)?; },
                    ProjectAction::RunWSBBuilder(wsb_execution) => { results.append(&mut wsb_execution.run(&mut self.project)?); },
                }
                Ok(())
            })?;
        Ok(results)
    }
}
