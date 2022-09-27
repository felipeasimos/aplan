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
    use crate::{builder::project_execution::ProjectExecution, task::{task_id::TaskId, Task}};


    #[test]
    fn builder_pattern() {
        let project = ProjectExecution::new("test")
            .wsb(|wsb| {
                wsb.expand(&[
                    ("", "Create WSB"),
                        ("1", "Create Task Tree"),
                        ("1", "CRUD"),
                            ("1.2", "Manage ac and pv"),
                    ("", "Create Burndown"),
                        ("2", "Plot graph"),
                        ("2", "Create story backlog")
                ])
                .value(&TaskId::new(vec![1, 2, 1]), 5.6)
                .done(&TaskId::new(vec![1, 1]), 0.4)
                .add(&TaskId::new(vec![]), "Create Web Interface");
            })
            .save()
            .run();
        // structure
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1])), Some(&Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 1])), Some(&Task::new(TaskId::new(vec![1, 1]), "Create Task Tree")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2])), Some(&Task::new(TaskId::new(vec![1, 2]), "CRUD")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2, 1])), Some(&Task::new(TaskId::new(vec![1, 2, 1]), "Manage ac and pv")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![2])), Some(&Task::new(TaskId::new(vec![2]), "Create Burndown")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![2, 1])), Some(&Task::new(TaskId::new(vec![2, 1]), "Plot graph")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![2, 2])), Some(&Task::new(TaskId::new(vec![2, 2]), "Create story backlog")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![3])), Some(&Task::new(TaskId::new(vec![3]), "Create Web Interface")));

        // value
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2, 1])).unwrap().get_planned_value(), 5.6);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2])).unwrap().get_planned_value(), 5.6);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1])).unwrap().get_planned_value(), 5.6);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 1])).unwrap().get_planned_value(), 0.0);

        // actual cost
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 1])).unwrap().get_actual_cost(), 0.4);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1])).unwrap().get_actual_cost(), 0.4);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2, 1])).unwrap().get_actual_cost(), 0.0);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2])).unwrap().get_actual_cost(), 0.0);


    }
}
