use diesel::prelude::*;
use chrono::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::contexts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Context {
	pub unique_id: i64,
	pub name: String,
	pub parent_id: Option<i64>,
	pub status: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub notes: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::folders)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Folder {
	pub unique_id: i64,
	pub name: String,
	pub parent_id: Option<i64>,
	pub status: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub notes: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Project {
	pub unique_id: i64,
	pub name: String,
	pub parent_id: Option<i64>,
	pub status: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub notes: Option<String>,
    pub context_id: Option<i64>,
    pub folder_id: Option<i64>,
    pub flagged: bool,
    pub deferred: Option<NaiveDateTime>,
    pub due: Option<NaiveDateTime>,
    pub is_repeating: bool,
    pub repeat_from: String,
    pub repeat_schedule: String,
    pub complete_with_last: bool,
    pub review_schedule: String,
    pub project_type: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Task {
	pub unique_id: i64,
	pub name: String,
	pub parent_id: Option<i64>,
	pub status: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub notes: Option<String>,
    pub context_id: Option<i64>,
    pub project_id: Option<i64>,
    pub flagged: bool,
    pub deferred: Option<NaiveDateTime>,
    pub due: Option<NaiveDateTime>,
    pub is_repeating: bool,
    pub repeat_from: String,
    pub repeat_schedule: String,
    pub task_type: String,
}

// Insertable Structs - Mapping struct columns in database
/******************************************************************************/
use crate::schema::*;

// ContextForm
/******************************************************************************/
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = contexts)]
pub struct ContextForm<'a> {
    pub name: &'a str,
    pub parent_id: Option<i64>,
    pub status: Option<&'a str>, // Default = "Active"
    pub notes: Option<&'a str>,
}

// FolderForm
/******************************************************************************/
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = folders)]
pub struct FolderForm<'a> {
    pub name: &'a str,
    pub parent_id: Option<i64>,
    pub status: Option<&'a str>, // Default = "Active"
    pub notes: Option<&'a str>,
}

// ProjectForm
/******************************************************************************/
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects)]
pub struct ProjectForm<'a> {
    pub name: &'a str,
    pub parent_id: Option<i64>,
    pub status: Option<&'a str>, // Default = "Active"
    pub notes: Option<&'a str>,
    pub context_id: Option<i64>,
    pub folder_id: Option<i64>,
    pub flagged: bool,
    pub deferred: Option<NaiveDateTime>,
    pub due: Option<NaiveDateTime>,
    pub is_repeating: bool,
    pub repeat_from: &'a str,
    pub repeat_schedule: &'a str,
    pub complete_with_last: bool,
    pub review_schedule: &'a str,
    pub project_type: &'a str,
}

// TasksForm
/******************************************************************************/
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tasks)]
pub struct TaskForm<'a> {
    pub name: &'a str,
    pub parent_id: Option<i64>,
    pub status: Option<&'a str>, // Default = "Active"
    pub notes: Option<&'a str>,
    pub context_id: Option<i64>,
    pub project_id: Option<i64>,
    pub flagged: bool,
    pub deferred: Option<NaiveDateTime>,
    pub due: Option<NaiveDateTime>,
    pub is_repeating: bool,
    pub repeat_from: &'a str,
    pub repeat_schedule: &'a str,
    pub task_type: &'a str,
}

