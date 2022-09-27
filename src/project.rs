use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use std::io::Write;

use crate::subsystem::wsb::WSB;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {

    wsb: WSB,
    // burndown: Burndown
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            wsb: WSB::new(name)
        }
    }

    pub fn wsb(&mut self) -> &mut WSB {
        &mut self.wsb
    }

    pub fn name(&self) -> &str {
        self.wsb.name()
    }

    pub fn load(name: &str) -> Option<Self> {
        let json_contents = Self::project_file_contents(&name).unwrap();
        Self::from_json(&json_contents).ok()
    }

    pub fn save(&self) -> Option<()> {
        let filename = Self::filename_from_project_name(self.name()).unwrap();
        self.save_to(&filename)
    }

    pub fn save_to(&self, filename: &str) -> Option<()> {
        let mut file = std::fs::File::create(filename).ok().unwrap();
        write!(file, "{}", self.to_json().ok().unwrap()).ok()
    }

    fn from_json(project_str: &str) -> Result<Self, serde_json::Error> {
        let project : Project = serde_json::from_str(project_str)?;
        Ok(project)
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn filename_from_project_name(name: &str) -> Option<String> {

        let filename = ".".to_string() + &name;
        let mut filename : PathBuf = PathBuf::from_str(&filename).ok()?;
        filename.set_extension("ap");
        Some(filename.to_str()?.to_string())
    }

    fn project_name_from_filename(filename: &str) -> Option<String> {

        let filename = PathBuf::from_str(filename).ok()?;
        let name : String = filename.file_stem()?.to_str()?[1..].to_string();
        Some(name)
    }

    fn project_file_contents(name: &str) -> Option<String> {
        let filename = Self::filename_from_project_name(name).unwrap();
        Some(std::fs::read_to_string(filename).unwrap())
    }
}

#[cfg(test)]
mod tests {

}
