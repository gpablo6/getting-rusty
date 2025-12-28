use std::env;

use chrono::{NaiveDateTime};
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

use crate::schema::task;
use crate::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = task)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[diesel(table_name = task)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub completed: bool,
}


pub struct DB {
    pub pool: Pool<ConnectionManager<SqliteConnection>>
}


pub fn retrieve_db_creds() -> String {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // SqliteConnection::establish(&database_url)
    //     .unwrap_or_else(|e| panic!("Error connecting to {}: {}", database_url, e))
    return database_url
}

impl DB {
    pub fn new() -> Self {
        let database_url = retrieve_db_creds();
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Self { pool }
    }

    pub fn add_task(
        &self,
        title: &str
    ) -> Result<Task> {
        let mut conn = self.pool.get()?;

        let new_task = NewTask { title, completed: false };
        
        let task = diesel::insert_into(task::table)
            .values(&new_task)
            .returning(Task::as_returning())
            .get_result(&mut conn)?;

        Ok(task)
    }

    pub fn get_task(
        &self,
        task_id: i32
    ) -> Result<Option<Task>> {
        use crate::schema::task::dsl::*;

        let mut conn = self.pool.get()?;

        let t = task
            .filter(id.eq(task_id))
            .select(Task::as_select())
            .first(&mut conn)
            .optional()?;

        Ok(t)
    }

    pub fn get_all_tasks(
        &self
    ) -> Result<Vec<Task>> {
        use crate::schema::task::dsl::*;

        let mut conn = self.pool.get()?;

        let tasks = task
            .select(Task::as_select())
            .load(&mut conn)?;

        Ok(tasks)
    }
}
