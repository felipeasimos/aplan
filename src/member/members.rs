use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::prelude::{Error, Tasks, TaskId};

use super::Member;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Members {
    #[serde_as(as="Vec<(_, _)>")]
    members: HashMap<String, Member>
}

impl Members {

    pub(crate) fn new() -> Self {
        Self {
            members: HashMap::new()
        }
    }

    pub fn get(&self, name: &str) -> Result<&Member, Error> {
        self.members.get(name)
            .ok_or_else(|| Error::MemberNotFound(name.to_string()))
    }

    pub fn members(&self) -> impl Iterator<Item=&Member> {
        self.members.values()
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub(crate) fn insert(&mut self, name: String) -> Result<(), Error> {
        self.members.insert(name.to_string(), Member::new(&name));
        Ok(())
    }

    pub(crate) fn remove(&mut self, name: &str) -> Result<Member, Error> {
        self.members.remove(&name[..])
            .ok_or_else(|| Error::MemberNotFound(name.to_string()))
    }

    pub(crate) fn get_mut(&mut self, name: &str) -> Result<&mut Member, Error> {
        self.members.get_mut(name)
            .ok_or_else(|| Error::MemberNotFound(name.to_string()))
    }

    pub(crate) fn assign_task_to_member(&mut self, id: TaskId, name: &str, tasks: &mut Tasks) -> Result<(), Error> {

        if tasks.get(&id)?.is_trunk() {
            return Err(Error::TrunkCannotAddMember(id.clone()))
        }

        self.get_mut(name)?.add_task(id);
        Ok(())
    }

    pub(crate) fn remove_member_from_task(&mut self, id: &TaskId, name: &str, tasks: &mut Tasks) -> Result<(), Error> {
        let task = tasks.get(id)?;
        if task.is_trunk() {
            return Err(Error::TrunkCannotRemoveMember(id.clone()))
        } else if !self.get(name)?.is_assigned_to(id) {
            return Err(Error::CannotRemoveMemberFromTask(id.clone(), name.to_string()))
        }
        Ok(self.get_mut(name)?.remove_task(id))
    }

    pub(crate) fn remove_member(&mut self, name: &str, tasks: &mut Tasks) -> Result<Member, Error> {

        self.get(name)?
            .task_ids()
            .cloned()
            .collect::<Vec<_>>()
            .iter()
            .try_for_each(|id| {
                self.remove_member_from_task(&id, name, tasks)
            })?;
        self.remove(name)
    }
}
