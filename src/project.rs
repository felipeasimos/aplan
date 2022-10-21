use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use std::io::Write;

use crate::{prelude::{Tasks, Members, Error, TaskId, Member}, interface::{task_execution::TaskExecution, member_execution::MemberExecution}, sprint::sprint::Sprints};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {

    pub(crate) tasks: Tasks,
    pub(crate) members: Members,
    pub(crate) sprints: Sprints
}

impl Project {

    pub fn new(name: &str) -> Self {
        Self {
            tasks: Tasks::new(name),
            members: Members::new(),
            sprints: Sprints::new()
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
        self.tasks.name()
    }

    pub fn tasks(&self) -> &Tasks {
        &self.tasks
    }

    pub fn members(&self) -> &Members {
        &self.members
    }

    pub fn tasks_mut<F>(&mut self, mut func: F) -> Result<&mut Self, Error>
    where F: FnMut(&mut TaskExecution<'_>) -> Result<(), Error> {
        {
            let mut wsb_execution = TaskExecution::new(self);
            func(&mut wsb_execution)?;
        }
        Ok(self)
    }

    pub fn members_mut<F>(&mut self, mut func: F) -> Result<&mut Self, Error>
    where F: FnMut(&mut MemberExecution<'_>) -> Result<(), Error> {
        {
            let mut member_execution = MemberExecution::new(self);
            func(&mut member_execution)?;
        }
        Ok(self)
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
