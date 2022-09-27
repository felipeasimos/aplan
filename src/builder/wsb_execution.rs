use crate::{task::task_id::TaskId, subsystem::wsb::WSB};

#[derive(Debug)]
enum WSBAction {
    Show(String),
    Add(TaskId, String),
    Done(TaskId, f64)
}

#[derive(Debug)]
pub struct WSBExecution {

    actions: Vec<WSBAction>
}

impl WSBExecution {

    pub(crate) fn new() -> WSBExecution {
        Self {
            actions: Vec::new()
        }
    }

    pub fn show(&mut self, filename: &str) -> &mut Self {
        self.actions.push(WSBAction::Show(filename.to_string()));
        self
    }

    pub fn add(&mut self, id: &TaskId, name: &str) -> &mut Self {
        self.actions.push(WSBAction::Add(id.clone(), name.to_string()));
        self
    }

    pub fn done(&mut self, id: &TaskId, cost: f64) -> &mut Self {
        self.actions.push(WSBAction::Done(id.clone(), cost));
        self
    }

    pub(crate) fn run(self, wsb: &mut WSB) {
        self.actions
            .into_iter()
            .for_each(|action| match action {
                WSBAction::Show(filename) => { wsb.to_dot_file(&filename); },
                WSBAction::Add(parent_id, name) => { wsb.add_task(&parent_id.to_string(), &name).unwrap(); },
                WSBAction::Done(id, cost) => { wsb.set_actual_cost(&id.to_string(), cost).unwrap(); },
            });
    }
}
