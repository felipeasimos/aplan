use crate::wsb::WSB;

pub struct Project {

    name: String,
    wsb: WSB,
    // burndown: Burndown
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            wsb: WSB::new(name)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_project() {

        let project = Project::new("Create Aplan");
        assert_eq!(project.name(), "Create Aplan");
    }
}
