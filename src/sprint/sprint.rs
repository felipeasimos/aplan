use chrono::{Utc, DateTime, Date, Duration};
use serde::{Deserialize, Serialize};

use crate::prelude::TaskId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Sprint {
    backlog: Vec<TaskId>,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Sprints {
    sprints: Vec<Sprint>,
    duration_in_weeks: u64,
    next_sprint: usize,
    start_date: DateTime<Utc>
}

impl Sprints {
    pub(crate) fn new() -> Self {
        Self {
            sprints: Vec::new(),
            duration_in_weeks: 0,
            next_sprint: 0,
            start_date: chrono::Utc::now()
        }
    }

    pub(crate) fn sprints(&self) -> &Vec<Sprint> {
        &self.sprints
    }

    pub(crate) fn sprint_to_date_time(&self, i: usize) -> (Date<Utc>, Date<Utc>) {
        let start = self.start_date + Duration::weeks((self.duration_in_weeks as i64) * (i as i64));
        let end = start + Duration::weeks(self.duration_in_weeks as i64);
        (start.date(), end.date())
    }

    pub(crate) fn sprints_date_times(&self) -> impl Iterator<Item=(Date<Utc>, Date<Utc>)> + '_ {
        self.sprints
            .iter()
            .enumerate()
            .map(|(i, _)| self.sprint_to_date_time(i))
    }

    pub(crate) fn product_backlog(&self) -> impl Iterator<Item=&TaskId> {

        self.sprints
            .iter()
            .flat_map(|sprint| &sprint.backlog)
    }
}
