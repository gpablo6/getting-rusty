use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}


impl From<crate::db::Task> for TaskResponse {
    fn from(task: crate::db::Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            completed: task.completed,
            created_at: Utc.from_utc_datetime(&task.created_at),
        }
    }
}
