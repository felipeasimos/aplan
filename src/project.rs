use std::{path::PathBuf, str::FromStr, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use std::io::Write;

use crate::{subsystem::wsb::WSB, error::Error, task::{task_id::TaskId, tasks::Tasks}, member::{Member, members::Members}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {

    pub(crate) wsb: WSB,
    pub(crate) tasks: Tasks,
    pub(crate) members: Members
    // burndown: Burndown
}

impl Project {
    pub(crate) fn new(name: &str) -> Self {
        let mut tasks = Tasks::new();
        let wsb = WSB::new(name, &mut tasks);
        Self {
            wsb,
            tasks,
            members: Members::new()
        }
    }

    pub(crate) fn assign_task_to_member(&mut self, id: TaskId, name: &str) -> Result<(), Error> {
        self.wsb.assign_task_to_member(&id, name, &mut self.tasks)?;

        Ok(self.members.get_mut(name)?.add_task(id))
    }

    pub(crate) fn remove_member_from_task(&mut self, id: &TaskId, name: &str) -> Result<(), Error> {
        self.wsb.remove_member_from_task(&id, name, &mut self.tasks)?;
        Ok(self.members.get_mut(name)?.remove_task(id))
    }

    pub(crate) fn remove_member(&mut self, name: &str) -> Result<(), Error> {

        self.members.get(name)?
            .clone()
            .task_ids()
            .try_for_each(|id| {
                self.remove_member_from_task(&id, name)
            })?;
        self.members.remove(name)?;
        Ok(())
    }

    pub(crate) fn name(&self) -> &str {
        self.wsb.name(&self.tasks)
    }

    pub(crate) fn load(name: &str) -> Result<Self, Error> {
        let json_contents = Self::project_file_contents(&name)?;
        Self::from_json(&json_contents)
    }

    pub(crate) fn save(&self) -> Result<(), Error> {
        let filename = Self::filename_from_project_name(self.name())?;
        self.save_to(&filename)
    }

    pub(crate) fn save_to(&self, filename: &str) -> Result<(), Error> {
        let mut file = std::fs::File::create(filename)
            .or_else(|_| Err(Error::OpenFile(filename.to_string())))?;
        write!(file, "{}", self.to_json()?)
            .or_else(|_| Err(Error::FileWrite(filename.to_string())))
    }

    pub(crate) fn from_json(project_str: &str) -> Result<Self, Error> {
        serde_json::from_str(project_str)
            .or_else(|_| Err(Error::ParseJsonContents(project_str.to_string())))
    }

    pub(crate) fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self)
            .or_else(|_| Err(Error::ParseProjectContents))
    }

    pub(crate) fn filename_from_project_name(name: &str) -> Result<String, Error> {

        let filename = ".".to_string() + &name;
        let mut filename : PathBuf = PathBuf::from_str(&filename)
            .or_else(|_| Err(Error::FileNotFound(filename.to_string())))?;
        filename.set_extension("ap");

        // TODO: check safety of this unwrap
        Ok(filename.to_str().unwrap().to_string())
    }

    pub(crate) fn project_name_from_filename(filename: &str) -> Result<String, Error> {

        let filename = PathBuf::from_str(filename)
            .or_else(|_| Err(Error::FileNotFound(filename.to_string())))?;
        // TODO: check safety of this unwrap
        let name : String = filename.file_stem().unwrap()
            .to_str().unwrap()[1..].to_string();
        Ok(name)
    }

    pub(crate) fn project_file_contents(name: &str) -> Result<String, Error> {
        let filename = Self::filename_from_project_name(name)?;
        Ok(std::fs::read_to_string(filename.clone())
            .or_else(|_| Err(Error::FileRead(filename.to_string())))?)
    }
}

#[cfg(test)]
mod tests {
}
