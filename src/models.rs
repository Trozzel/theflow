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

// ADDING NEW RECORDS TO TABLES
/******************************************************************************/
use crate::schema::*;

// NEW POSTS
#[derive(Insertable)]
#[diesel(table_name = contexts)]
pub struct NewContext<'a> {
    pub name: &'a str,
    pub parent_id: Option<i64>,
    pub status: &'a str,
    pub notes: Option<&'a str>,
}
