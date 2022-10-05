use crate::task::task_id::TaskId;

enum MemberAction {
    AddMember(String),
    RemoveMember(String),
    AssignTaskToMember(TaskId, String),
    RemoveMemberFromTask(TaskId, String),
}
