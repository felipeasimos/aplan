use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use std::io::Write;

use crate::{subsystem::wsb::WSB, error::Error, task::{task_id::TaskId, tasks::Tasks}, member::members::Members, builder::{wsb_execution::WSBExecution, member_execution::MemberExecution}, prelude::Member};

pub type Aplan = Project;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {

    pub(crate) wsb: WSB,
    pub(crate) tasks: Tasks,
    pub(crate) members: Members
    // burndown: Burndown
}

impl Project {

    pub fn new(name: &str) -> Self {
        let mut tasks = Tasks::new();
        let wsb = WSB::new(name, &mut tasks);
        Self {
            wsb,
            tasks,
            members: Members::new()
        }
    }

    pub fn load(name: &str) -> Result<Self, Error> {
        let json_contents = Self::project_file_contents(&name)?;
        Self::from_json(&json_contents)
    }

    pub fn load_from(filename: &str) -> Result<Self, Error> {
        let json_contents = std::fs::read_to_string(filename.clone())
            .or_else(|_| Err(Error::FileRead(filename.to_string())))?;
        Self::from_json(&json_contents)
    }

    pub fn save(&mut self) -> Result<&mut Self, Error> {
        let filename = Self::filename_from_project_name(self.name())?;
        self.save_to(&filename)
    }

    pub fn save_to(&mut self, filename: &str) -> Result<&mut Self, Error> {
        let mut file = std::fs::File::create(filename)
            .or_else(|_| Err(Error::OpenFile(filename.to_string())))?;
        write!(file, "{}", self.to_json()?)
            .or_else(|_| Err(Error::FileWrite(filename.to_string())))?;
        Ok(self)
    }

    pub fn name(&self) -> &str {
        self.wsb.name(&self.tasks)
    }

    pub fn wsb<F>(&mut self, mut func: F) -> Result<&mut Self, Error>
    where F: FnMut(&mut WSBExecution<'_>) -> Result<(), Error> {
        {
            let mut wsb_execution = WSBExecution::new(self);
            func(&mut wsb_execution)?;
        }
        Ok(self)
    }

    pub fn members<F>(&mut self, mut func: F) -> Result<&mut Self, Error>
    where F: FnMut(&mut MemberExecution<'_>) -> Result<(), Error> {

        {
            let mut member_execution = MemberExecution::new(self);
            func(&mut member_execution)?;
        }
        Ok(self)
    }

    pub(crate) fn assign_task_to_member(&mut self, id: TaskId, name: &str) -> Result<(), Error> {
        self.wsb.assign_task_to_member(&id, name, &mut self.tasks)?;

        Ok(self.members.get_mut(name)?.add_task(id))
    }

    pub(crate) fn remove_member_from_task(&mut self, id: &TaskId, name: &str) -> Result<(), Error> {
        self.wsb.remove_member_from_task(&id, name, &mut self.tasks)?;
        Ok(self.members.get_mut(name)?.remove_task(id))
    }

    pub(crate) fn remove_member(&mut self, name: &str) -> Result<Member, Error> {

        self.members.get(name)?
            .clone()
            .task_ids()
            .try_for_each(|id| {
                self.remove_member_from_task(&id, name)
            })?;
        self.members.remove(name)
    }

    fn from_json(project_str: &str) -> Result<Self, Error> {
        serde_json::from_str(project_str)
            .or_else(|_| Err(Error::ParseJsonContents(project_str.to_string())))
    }

    fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self)
            .or_else(|_| Err(Error::ParseProjectContents))
    }

    fn filename_from_project_name(name: &str) -> Result<String, Error> {

        let filename = ".".to_string() + &name;
        let mut filename : PathBuf = PathBuf::from_str(&filename)
            .or_else(|_| Err(Error::FileNotFound(filename.to_string())))?;
        filename.set_extension("ap");

        // TODO: check safety of this unwrap
        Ok(filename.to_str().unwrap().to_string())
    }

    fn project_file_contents(name: &str) -> Result<String, Error> {
        let filename = Self::filename_from_project_name(name)?;
        Ok(std::fs::read_to_string(filename.clone())
            .or_else(|_| Err(Error::FileRead(filename.to_string())))?)
    }
}

#[cfg(test)]
mod tests {
}
