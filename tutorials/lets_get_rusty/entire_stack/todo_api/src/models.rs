// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};


// #[derive(Debug, Serialize, Deserialize)]
// pub struct TaskResponse {
//     pub id: i32,
//     pub title: String,
//     pub completed: bool,
//     pub created_at: DateTime<Utc>,
// }


// impl From<crate::db::Task> for TaskResponse {
//     fn from(task: crate::db::Task) -> Self {
//         TaskResponse {
//             id: task.id,
//             title: task.title,
//             completed: task.completed,
//             created_at: DateTime::from_utc(task.created_at, Utc),
//         }
//     }
// }
