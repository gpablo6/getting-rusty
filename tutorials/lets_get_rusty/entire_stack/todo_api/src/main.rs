use rocket::{get, launch, routes, serde::json::Json, State};

use crate::db::{DB};
// use crate::models::{TaskResponse};

mod db;
mod error;
mod prelude;
mod schema;
// mod models;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[get("/task/<id>")]
// fn get_task(id: i32, db: &State<DB>) -> Result<Json<Task>, error::Error> {
//     let task = db.get_task(id)?;

//     Ok(Json(task))
// }

// #[get("/tasks")]
// fn get_all_tasks(db: &State<DB>) -> Result<Json<Vec<TaskResponse>>, error::Error> {
//     let tasks = db.get_all_tasks()?;
//     let task_responses: Vec<TaskResponse> = tasks.into_iter().map(|t| t.into()).collect();
//     Ok(Json(tasks))
// }

// #[post("/task/<title>")]
// fn add_task(title: String, db: &State<DB>) -> Result<Json<db::Task>, error::Error> {
//     let task = db.add_task(&title)?;
//     Ok(Json(task))
// }

#[launch]
fn rocket() -> _ {
    let _db = DB::new();

    rocket::build()
        .manage(_db)
        .mount("/", routes![index])
}
