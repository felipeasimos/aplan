use crate::task::task_id::TaskId;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {

    #[error("Task with id {0} not found")]
    TaskNotFound(TaskId),

    #[error("Not a valid TaskId string {0}")]
    BadTaskIdString(String),

    #[error("File not found {0}")]
    FileNotFound(String),

    #[error("Trunk tasks like {0} cannot be removed")]
    TrunkCannotBeRemoved(TaskId),

    #[error("Root task '{0}' doesn't have a parent")]
    NoParent(TaskId),

    #[error("Root task '{0}' doesn't have a child index")]
    NoChildIndex(TaskId),

    #[error("Can't change actual cost of trunk tasks like {0} directly")]
    TrunkCannotChangeCost(TaskId),

    #[error("Can't change planned value of trunk tasks like {0} directly")]
    TrunkCannotChangeValue(TaskId)
}
