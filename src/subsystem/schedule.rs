use chrono::{Utc, Weekday, Duration, DateTime};
use serde::{Serialize, Deserialize};

mod custom_date {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{Serializer, Deserializer, Deserialize};

    const FORMAT: &'static str = "%Y-%m-%d";
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    weekday: Weekday,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    available: Duration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum RoutineExceptionType {
    AVAILABLE,
    OCCUPIED
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineException {
    t: RoutineExceptionType,
    date: DateTime<Utc>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    duration: Duration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineExceptions {
    exceptions: Vec<RoutineException>
}

impl RoutineExceptions {
    fn new() -> Self {
        Self {
            exceptions: Vec::new()
        }
    }
}

/// Schedule of a member of the project.
/// A basic routine is set, and exceptions to it can be inserted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(with="custom_date")]
    start: chrono::DateTime<Utc>,
    routine: Vec<TimeSlot>,
    sprint_in_weeks: u32,
    exceptions: RoutineExceptions
}

impl Schedule {

    pub fn new() -> Self {
        Self {
            start: chrono::Utc::now(),
            routine: Vec::new(),
            sprint_in_weeks: 2,
            exceptions: RoutineExceptions::new()
        }
    }

    pub fn set_sprint_in_weeks(&mut self, weeks: u32) -> &mut Self {
        self.sprint_in_weeks = weeks;
        self
    }

    pub fn set_start(&mut self, start: chrono::DateTime<Utc>) -> &mut Self {
        self.start = start;
        self
    }
}
