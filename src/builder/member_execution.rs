use crate::{task::task_id::TaskId, error::Error, project::Project, member::Member};

use super::project_execution::Return;

#[derive(Debug, Clone)]
enum MemberAction {
    ListMembers,
    GetMember(String),
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

    pub(crate) fn new() -> Self {
        Self {
            actions: Vec::new()
        }
    }

    pub fn list_members(&mut self) -> &mut Self {
        self.actions.push(MemberAction::ListMembers);
        self
    }

    pub fn get_member(&mut self, name: &str) -> &mut Self {
        self.actions.push(MemberAction::GetMember(name.to_string()));
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

    pub(crate) fn run(self, project: &mut Project) -> Result<Vec<Return>, Error> {
        Ok(self.actions
            .into_iter()
            .map(|action| -> Result<Option<Return>, Error> {
                Ok(match action {
                    MemberAction::ListMembers => { Some(Return::MembersList(project.members().collect::<Vec<Member>>())) },
                    MemberAction::GetMember(name) => { Some(Return::Member(project.get_member(&name)?.clone())) },
                    MemberAction::AddMember(name) => { project.add_member(&name); None },
                    MemberAction::RemoveMember(name) => { project.remove_member(&name)?; None },
                    MemberAction::AssignTaskToMember(task_id, name) => { project.assign_task_to_member(task_id, &name)?; None },
                    MemberAction::RemoveMemberFromTask(task_id, name) => { project.remove_member_from_task(&task_id, &name)?; None },
                })
            })
            .collect::<Result<Vec<Option<Return>>, Error>>()?
            .into_iter()
            .filter_map(|res| res)
            .collect::<Vec<Return>>())
    }
}
