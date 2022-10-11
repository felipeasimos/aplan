use crate::{task::task_id::TaskId, error::Error, project::Project, member::Member};

use super::project_execution::Return;

#[derive(Debug, Clone)]
enum MemberAction {
    ListMembers,
    AddMember(String),
    RemoveMember(String),
    AssignTaskToMember(TaskId, String),
    RemoveMemberFromTask(TaskId, String),
}

#[derive(Debug, Clone)]
pub struct MemberExecution {
    actions: Vec<MemberAction>,
}

impl MemberExecution {

    pub fn new() -> Self {
        Self {
            actions: Vec::new()
        }
    }

    pub fn list_members(&mut self) -> &mut Self {
        self.actions.push(MemberAction::ListMembers);
        self
    }

    pub fn add_member(&mut self, name: &str) -> &mut Self {
        self.actions.push(MemberAction::AddMember(name.to_string()));
        self
    }

    pub fn remove_member(&mut self, name: &str) -> &mut Self {
        self.actions.push(MemberAction::RemoveMember(name.to_string()));
        self
    }

    pub fn assign_task_to_member(&mut self, id: TaskId, name: &str) -> &mut Self {
        self.actions.push(MemberAction::AssignTaskToMember(id, name.to_string()));
        self
    }

    pub fn remove_member_from_task(&mut self, id: TaskId, name: &str) -> &mut Self {
        self.actions.push(MemberAction::RemoveMemberFromTask(id, name.to_string()));
        self
    }

    pub fn run(self, project: &mut Project) -> Result<Vec<Return>, Error> {
        let mut results : Vec<Return> = Vec::new();
        self.actions
            .into_iter()
            .try_for_each(|action| -> Result<(), Error> {
                match action {
                    MemberAction::ListMembers => { results.push(Return::MembersList(project.members().collect::<Vec<Member>>())); },
                    MemberAction::AddMember(name) => { project.add_member(&name); },
                    MemberAction::RemoveMember(name) => { project.remove_member(&name)?; },
                    MemberAction::AssignTaskToMember(task_id, name) => { project.assign_task_to_member(task_id, &name)?; },
                    MemberAction::RemoveMemberFromTask(task_id, name) => { project.remove_member_from_task(&task_id, &name)?; },
                }
                Ok(())
            })?;
        Ok(results)
    }
}
