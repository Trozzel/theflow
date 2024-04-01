use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema;
use crate::models;

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
    diesel::insert_into(schema::contexts::table)
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
    diesel::insert_into(schema::folders::table)
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
    diesel::insert_into(schema::projects::table)
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
    diesel::insert_into(schema::tasks::table)
        .values(&new_task)
        .execute(conn)
}

