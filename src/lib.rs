pub mod cli;
pub mod models;
pub mod schema;
pub mod traits;
pub mod helpers;

//use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::{contexts, folders, projects, tasks};
use std::env;

/// Establish and return the database connection
/******************************************************************************/
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// INSERT SINGLE CONTEXT
/******************************************************************************/
pub fn insert_context(
    conn: &mut MysqlConnection,
    name: &str,
    parent_id: Option<i64>,
    status: Option<&str>,
    notes: Option<&str>,
) -> QueryResult<usize> {
    let new_context = models::ContextForm {
        name,
        parent_id,
        status,
        notes,
    };
    diesel::insert_into(contexts::table)
        .values(&new_context)
        .execute(conn)
}


// INSERT SINGLE FOLDER
/******************************************************************************/
pub fn insert_folder(
    conn: &mut MysqlConnection,
    name: &str,
    parent_id: Option<i64>,
    status: Option<&str>,
    notes: Option<&str>,
) -> QueryResult<usize> {
    let new_folder = models::FolderForm {
        name,
        parent_id,
        status,
        notes,
    };
    diesel::insert_into(folders::table)
        .values(&new_folder)
        .execute(conn)
}

// INSERT SINGLE PROJECT
/******************************************************************************/
pub fn insert_project(
    conn: &mut MysqlConnection,
    name: &str,
    parent_id: Option<i64>,
    status: Option<&str>,
    notes: Option<&str>,
    context_id: Option<i64>,
    folder_id: Option<i64>,
    flagged: bool,
    deferred: Option<NaiveDateTime>,
    due: Option<NaiveDateTime>,
    is_repeating: bool,
    repeat_from: &str,
    repeat_schedule: &str,
    complete_with_last: bool,
    review_schedule: &str,
    project_type: &str,
) -> QueryResult<usize> {
    let new_project = models::ProjectForm {
        name,
        parent_id,
        status,
        notes,
        context_id,
        folder_id,
        flagged,
        deferred,
        due,
        is_repeating,
        repeat_from,
        repeat_schedule,
        complete_with_last,
        review_schedule,
        project_type,
    };
    diesel::insert_into(projects::table)
        .values(&new_project)
        .execute(conn)
}

// INSERT SINGLE TASK
/******************************************************************************/
pub fn insert_task(
    conn: &mut MysqlConnection,
    name: &str,
    parent_id: Option<i64>,
    status: Option<&str>,
    notes: Option<&str>,
    context_id: Option<i64>,
    project_id: Option<i64>,
    flagged: bool,
    deferred: Option<NaiveDateTime>,
    due: Option<NaiveDateTime>,
    is_repeating: bool,
    repeat_from: &str,
    repeat_schedule: &str,
    task_type: &str,
) -> QueryResult<usize> {
    let new_task = models::TaskForm {
        name,
        parent_id,
        status,
        notes,
        context_id,
        project_id,
        flagged,
        deferred,
        due,
        is_repeating,
        repeat_from,
        repeat_schedule,
        task_type,
    };
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
}
