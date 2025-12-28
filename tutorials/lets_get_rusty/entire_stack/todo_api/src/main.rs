use rocket::{get, post, launch, routes, serde::json::Json, State};

use crate::db::{DB};
use crate::models::{TaskResponse};
use crate::prelude::*;
use cors::*;

mod db;
mod error;
mod prelude;
mod schema;
mod models;
mod cors;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/task/<id>")]
fn get_task(id: i32, db: &State<DB>) -> ApiResult<Json<TaskResponse>> {
    let task = db.get_task(id).map_err(ApiError::from)?;

    let resp = task.ok_or_else(
        || ApiError::NotFound(format!("task {} not found", id))
    )?;

    Ok(Json(resp.into()))
}

#[get("/tasks")]
fn get_all_tasks(db: &State<DB>) -> ApiResult<Json<Vec<TaskResponse>>> {
    let tasks = db.get_all_tasks().map_err(ApiError::from)?;

    let resp: Vec<TaskResponse> = tasks.into_iter().map(|t| t.into()).collect();

    Ok(Json(resp))
}

#[post("/task/<title>")]
fn add_task(title: String, db: &State<DB>) -> ApiResult<Json<TaskResponse>> {
    let task = db.add_task(&title).map_err(ApiError::from)?;

    Ok(Json(task.into()))
}

#[launch]
fn rocket() -> _ {
    let _db = DB::new();

    rocket::build()
        .manage(_db)
        .attach(CORS)
        .mount("/", routes![index, get_task, get_all_tasks, add_task])
}
