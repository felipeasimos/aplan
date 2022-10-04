use std::{path::PathBuf, str::FromStr, collections::HashMap};

use serde::{Deserialize, Serialize, de::{Visitor, self}, ser::SerializeMap};

use std::io::Write;

use crate::{subsystem::{wsb::WSB, member::Member}, error::Error, task::tasks::Tasks};
#[derive(Debug, Clone)]
pub struct Members(HashMap<String, Member>);

impl Serialize for Members {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;
            for (k, v) in &self.0 {
                map.serialize_entry(&k.to_string(), &v)?;
            }
            map.end()
    }
}

struct MembersVisitor;

impl<'de> Visitor<'de> for MembersVisitor {
    type Value = Members;

    fn expecting(&self, formatter: &mut serde::__private::fmt::Formatter) -> serde::__private::fmt::Result {
        formatter.write_str("HashMap with String as key and Member as value")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut members : HashMap<String, Member> = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            members.insert(key, value);
        }

        Ok(Members(members))
    }
}

impl<'de> Deserialize<'de> for Members {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        deserializer.deserialize_map(MembersVisitor {})
    }
}


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
            members: Members(HashMap::new())
        }
    }

    pub fn wsb(&mut self) -> &mut WSB {
        &mut self.wsb
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
