use crate::{task::task_id::TaskId, prelude::{Project, Member, Error}};

#[derive(Debug)]
pub struct MemberExecution<'a> {
    project: &'a mut Project
}

impl<'a> MemberExecution<'a> {

    pub(crate) fn new(project: &'a mut Project) -> Self {
        Self {
            project
        }
    }

    pub fn list_members(&mut self) -> impl Iterator<Item=&Member> {
        self.project.members.members()
    }

    pub fn get_member(&mut self, name: &str) -> Result<&Member, Error> {
        self.project.members.get(name)
    }

    pub fn add_member(&mut self, name: &str) -> Result<&mut Self, Error> {
        self.project.members.insert(name.to_string())?;
        Ok(self)
    }

    pub fn remove_member(&mut self, name: &str) -> Result<Member, Error> {
        self.project.members.remove_member(name, &mut self.project.tasks)
    }

    pub fn assign_task_to_member(&mut self, id: TaskId, name: &str) -> Result<&mut Self, Error> {
        self.project.members.assign_task_to_member(id, name, &mut self.project.tasks)?;
        Ok(self)
    }

    pub fn remove_member_from_task(&mut self, id: TaskId, name: &str) -> Result<&mut Self, Error> {
        self.project.members.remove_member_from_task(&id, name, &mut self.project.tasks)?;
        Ok(self)
    }
}
