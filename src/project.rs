use serde::{Deserialize, Serialize};

use std::io::Write;

use crate::{prelude::{Tasks, Members, Error}, interface::{task_execution::TaskExecution, member_execution::MemberExecution}, sprint::sprint::Sprints, util::DEFAULT_FILENAME};

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

    pub fn load(filename: &str) -> Result<Self, Error> {
        let json_contents = std::fs::read_to_string(&filename)
            .or_else(|_| Err(Error::FileRead(filename.to_string())))?;
        Self::from_json(&json_contents)
    }

    pub fn save(&mut self) -> Result<&mut Self, Error> {
        self.save_to(DEFAULT_FILENAME)
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
}

#[cfg(test)]
mod tests {
}
