use std::option::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TaskId {
    id: Vec<u32>
}

impl TaskId {

    pub fn new(id: Vec<u32>) -> Self {
        Self { id }
    }

    pub fn as_vec(&self) -> &Vec<u32> {
        &self.id
    }

    pub fn as_vec_mut(&mut self) -> &mut Vec<u32> {
        &mut self.id
    }

    pub fn to_vec(self) -> Vec<u32> {
        self.id
    }

    pub fn parse(id: &str) -> Option<Self> {
        if id.is_empty() {
            return Some(TaskId::new(vec![]));
        }
        let vec = id
            .split(".")
            .map(|n| n.parse::<u32>().ok())
            .collect::<Option<Vec<u32>>>()?;
        Some(TaskId::new(vec))
    }

    pub fn parent(&self) -> Option<TaskId> {
        let vec_len = self.id.len();
        if vec_len < 1 {
            return None;
        }
        let parent_vec = self.id[..vec_len-1].to_vec();
        Some(TaskId::new(parent_vec))
    }

    pub fn child_ids(&self, num_childs: u32) -> Vec<TaskId> {

        let id_vec = self.as_vec();
        (1..num_childs+1).map(|child_index| {
            let mut vec = id_vec.clone();
            vec.push(child_index);
            TaskId::new(vec)
        }).collect::<Vec<TaskId>>()
    }

    pub fn path(&self) -> Vec<TaskId> {

        let id_iter = self.id
            .iter()
            .enumerate()
            .map(|(layer_idx, _)| {
                let mut id_vec = self.id.clone();
                id_vec.truncate(layer_idx + 1);
                TaskId::new(id_vec)
            });
        std::iter::once(Self::get_root_id())
            .chain(id_iter)
            .collect::<Vec<TaskId>>()
    }

    pub fn new_child_id(&self, child_num: u32) -> TaskId {
        let id_vec = self
            .as_vec()
            .iter()
            .cloned()
            .chain(std::iter::once(child_num))
            .collect::<Vec<u32>>();
        TaskId::new(id_vec)
    }

    pub fn get_root_id() -> TaskId {
        TaskId::new(vec![])
    }
}

impl ToString for TaskId {
    fn to_string(&self) -> String {
        self.id
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        let task_id = TaskId::new(vec![1,1]);
        assert_eq!(task_id.as_vec(), &vec![1,1]);
        assert_eq!(task_id.to_string(), "1.1");
        assert_eq!(TaskId::new(vec![1,2,3,4]).to_string(), "1.2.3.4");
    }

    #[test]
    fn parse() {
        assert_eq!(TaskId::parse("1.1").unwrap().as_vec(), &vec![1,1]);
        assert_eq!(TaskId::parse("4.523.123").unwrap().as_vec(), &vec![4, 523, 123]);
        assert!(TaskId::parse(".1.1").is_none());
        assert!(TaskId::parse("1.1.").is_none());
    }

    #[test]
    fn parent_id() {
        assert_eq!(TaskId::parse("1.1").unwrap().parent().unwrap().as_vec(), &vec![1]);
        assert_eq!(TaskId::parse("1.1.234.12").unwrap().parent().unwrap().as_vec(), &vec![1,1,234]);
        assert_eq!(TaskId::parse("2.534.234.12.243.123").unwrap().parent().unwrap().as_vec(), &vec![2, 534, 234, 12, 243]);
    }
}
