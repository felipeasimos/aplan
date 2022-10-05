use std::collections::{HashMap, hash_map::Values};

use serde::{Serialize, de::{Visitor, self}, Deserialize, ser::SerializeMap};

use crate::task::{Task, task_id::TaskId};

#[derive(Debug, Clone)]
pub struct Tasks(HashMap<TaskId, Task>);

impl Tasks {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }

    pub(crate) fn insert(&mut self, id: TaskId, task: Task) {
        self.0.insert(id, task);
    }

    pub(crate) fn get(&self, id: &TaskId) -> Option<&Task> {
        self.0.get(id)
    }

    pub(crate) fn get_mut(&mut self, id: &TaskId) -> Option<&mut Task> {
        self.0.get_mut(id)
    }

    pub(crate) fn values(&self) -> Values<TaskId, Task> {
        self.0.values()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn remove(&mut self, key: &TaskId) -> Option<Task> {
        self.0.remove(key)
    }
}

impl Serialize for Tasks {
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

struct TasksVisitor;

impl<'de> Visitor<'de> for TasksVisitor {
    type Value = Tasks;

    fn expecting(&self, formatter: &mut serde::__private::fmt::Formatter) -> serde::__private::fmt::Result {
        formatter.write_str("HashMap with TaskId as key and Task as value")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut tasks : HashMap<TaskId, Task> = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            let task_id = TaskId::parse(key).unwrap();
            tasks.insert(task_id, value);
        }

        Ok(Tasks(tasks))
    }
}

impl<'de> Deserialize<'de> for Tasks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        deserializer.deserialize_map(TasksVisitor {})
    }
}


