use serde::{Deserialize, Serialize};

use crate::wsb::WSB;

#[derive(Serialize, Deserialize)]
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

    pub fn from_json(project_str: &str) -> Result<Self, serde_json::Error> {
        let project : Project = serde_json::from_str(project_str)?;
        Ok(project)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {

}
