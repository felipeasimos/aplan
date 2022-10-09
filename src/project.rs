use std::{path::PathBuf, str::FromStr, collections::{HashMap, hash_map::Iter}};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use std::io::Write;

use crate::{subsystem::wsb::WSB, error::Error, task::{task_id::TaskId, Task}, member::Member};

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {

    pub(crate) wsb: WSB,
    #[serde_as(as = "Vec<(_, _)>")]
    pub(crate) tasks: HashMap<TaskId, Task>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub(crate) members: HashMap<String, Member>
    // burndown: Burndown
}

impl Project {
    pub fn new(name: &str) -> Self {
        let mut tasks = HashMap::new();
        let wsb = WSB::new(name, &mut tasks);
        Self {
            wsb,
            tasks,
            members: HashMap::new()
        }
    }

    pub fn get_member(&self, name: &str) -> Result<&Member, Error> {
        self.members.get(name)
            .ok_or_else(|| Error::MemberNotFound(name.to_string()))
    }

    pub fn get_member_mut(&mut self, name: &str) -> Result<&mut Member, Error> {
        self.members.get_mut(name)
            .ok_or_else(|| Error::MemberNotFound(name.to_string()))
    }

    pub fn members(&self) -> impl Iterator<Item=Member> + '_ {
        self.members.values().map(|v| v.clone())
    }

    pub fn add_member(&mut self, name: &str) {
        self.members.insert(name.to_string(), Member::new(&name));
    }

    pub fn remove_member(&mut self, name: &str) -> Result<(), Error> {

        self.get_member(name)?
            .clone()
            .tasks()
            .try_for_each(|id| {
                self.remove_member_from_task(&id, name)
            })?;
        self.members.remove(name);
        Ok(())
    }

    pub fn assign_task_to_member(&mut self, id: TaskId, name: &str) -> Result<(), Error> {
        self.wsb.assign_task_to_member(&id, name, &mut self.tasks)?;

        Ok(self.get_member_mut(name)?.add_task(id))
    }

    pub fn remove_member_from_task(&mut self, id: &TaskId, name: &str) -> Result<(), Error> {
        self.wsb.remove_member_from_task(&id, name, &mut self.tasks)?;
        Ok(self.get_member_mut(name)?.remove_task(id))
    }

    pub fn name(&self) -> &str {
        self.wsb.name(&self.tasks)
    }

    pub fn load(name: &str) -> Result<Self, Error> {
        let json_contents = Self::project_file_contents(&name)?;
        Self::from_json(&json_contents)
    }

    pub fn save(&self) -> Result<(), Error> {
        let filename = Self::filename_from_project_name(self.name())?;
        self.save_to(&filename)
    }

    pub fn save_to(&self, filename: &str) -> Result<(), Error> {
        let mut file = std::fs::File::create(filename)
            .or_else(|_| Err(Error::OpenFile(filename.to_string())))?;
        write!(file, "{}", self.to_json()?)
            .or_else(|_| Err(Error::FileWrite(filename.to_string())))
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

    fn project_name_from_filename(filename: &str) -> Result<String, Error> {

        let filename = PathBuf::from_str(filename)
            .or_else(|_| Err(Error::FileNotFound(filename.to_string())))?;
        // TODO: check safety of this unwrap
        let name : String = filename.file_stem().unwrap()
            .to_str().unwrap()[1..].to_string();
        Ok(name)
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
