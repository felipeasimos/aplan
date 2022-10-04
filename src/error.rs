use crate::{task::task_id::TaskId, project::Project};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {

    #[error("Task with id {0} not found")]
    TaskNotFound(TaskId),

    #[error("Not a valid TaskId string {0}")]
    BadTaskIdString(String),

    #[error("File not found {0}")]
    FileNotFound(String),

    #[error("Couldn't open file {0}")]
    OpenFile(String),

    #[error("Couldn't get stem from filename {0}")]
    FilenameStem(String),

    #[error("Couldn't read file {0}")]
    FileRead(String),

    #[error("Couldn't write to file {0}")]
    FileWrite(String),

    #[error("Trunk tasks like {0} cannot be removed")]
    TrunkCannotBeRemoved(TaskId),

    #[error("Root task '{0}' doesn't have a parent")]
    NoParent(TaskId),

    #[error("Root task '{0}' doesn't have a child index")]
    NoChildIndex(TaskId),

    #[error("Can't change actual cost of trunk tasks like {0} directly")]
    TrunkCannotChangeCost(TaskId),

    #[error("Can't change planned value of trunk tasks like {0} directly")]
    TrunkCannotChangeValue(TaskId),

    #[error("Couldn't parse JSON to project: {0}")]
    ParseJsonContents(String),

    #[error("Couldn't parse project to JSON")]
    ParseProjectContents,

    #[error("Couldn't parse CLI argument: {0}")]
    ParseCliArgument(String),

    #[error("There is no next sibling for: {0}")]
    NoNextSibling(TaskId),

    #[error("There is no prev sibling for: {0}")]
    NoPrevSibling(TaskId)
}
