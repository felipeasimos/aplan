use std::{path::PathBuf, str::FromStr, collections::HashMap};

use serde::{Deserialize, Serialize, de::{Visitor, self}, ser::SerializeMap};

use std::io::Write;

use crate::{subsystem::wsb::WSB, task::{Task, task_id::TaskId}};

pub(crate) struct Project {

    pub(crate) wsb: WSB,
    pub(crate) tasks: HashMap<TaskId, Task>
    // burndown: Burndown
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
            let mut map = serializer.serialize_map(Some(self.tasks.len()))?;
            for (k, v) in &self.tasks {
                map.serialize_entry(&k.to_string(), &v)?;
            }
            map.end()
    }
}

struct ProjectVisitor;

impl<'de> Visitor<'de> for ProjectVisitor {
    type Value = Project;

    fn expecting(&self, formatter: &mut serde::__private::fmt::Formatter) -> serde::__private::fmt::Result {
        formatter.write_str("HashMap with TaskId as key and Task as value")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut map : HashMap<TaskId, Task> = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            let task_id = TaskId::parse(key).unwrap();
            map.insert(task_id, value);
        }

        Ok(Project {
            wsb: WSB {},
            tasks: map
        })
    }
}

impl<'de> Deserialize<'de> for Project {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        deserializer.deserialize_map(ProjectVisitor {})
    }
}
impl Project {
    pub fn new(name: &str) -> Self {
        let mut map = HashMap::new();
        Self {
            wsb: WSB::new(name, &mut map),
            tasks: map
        }
    }

    pub fn wsb(&mut self) -> &mut WSB {
        &mut self.wsb
    }

    pub fn name(&self) -> &str {
        self.wsb.name(&self.tasks)
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
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1]), &project.tasks), Some(&Task::new(TaskId::new(vec![1]), "Create WSB")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 1]), &project.tasks), Some(&Task::new(TaskId::new(vec![1, 1]), "Create Task Tree")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2]), &project.tasks), Some(&Task::new(TaskId::new(vec![1, 2]), "CRUD")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2, 1]), &project.tasks), Some(&Task::new(TaskId::new(vec![1, 2, 1]), "Manage ac and pv")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![2]), &project.tasks), Some(&Task::new(TaskId::new(vec![2]), "Create Burndown")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![2, 1]), &project.tasks), Some(&Task::new(TaskId::new(vec![2, 1]), "Plot graph")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![2, 2]), &project.tasks), Some(&Task::new(TaskId::new(vec![2, 2]), "Create story backlog")));
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![3]), &project.tasks), Some(&Task::new(TaskId::new(vec![3]), "Create Web Interface")));

        // value
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2, 1]), &project.tasks).unwrap().get_planned_value(), 5.6);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2]), &project.tasks).unwrap().get_planned_value(), 5.6);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1]), &project.tasks).unwrap().get_planned_value(), 5.6);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 1]), &project.tasks).unwrap().get_planned_value(), 0.0);

        // actual cost
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 1]), &project.tasks).unwrap().get_actual_cost(), 0.4);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1]), &project.tasks).unwrap().get_actual_cost(), 0.4);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2, 1]), &project.tasks).unwrap().get_actual_cost(), 0.0);
        assert_eq!(project.wsb.get_task(&TaskId::new(vec![1, 2]), &project.tasks).unwrap().get_actual_cost(), 0.0);


    }
}
