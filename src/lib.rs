pub mod models;
pub mod schema;

//use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::contexts;
use std::env;
/// Establish and return the database connection 
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_context (conn: &mut MysqlConnection,
                    name: &str,
                    parent_id: Option<i64>,
                    status: Option<&str>,
                    notes: Option<&str>) -> QueryResult<usize>
{
    // 1. Establish the parent_id
    //let pid_str: String = match parent_id {
    //    Some(id) => id.to_string(),
    //    None => "null".to_string(),
    //};
    //let pid_str = &pid_str;

    //// 2. Establish the status. NOTE: default is "Active"
    //let status: String = match status {
    //    Some(stat) => stat.to_string(),
    //    None => "Active".to_string(),
    //};
    //let status = &status;

    //// 3. Establish the notes
    //let notes: String = match notes {
    //    Some(note) => note.to_string(),
    //    None => "null".to_string(),
    //};
    //let notes = &notes;

    let new_context = models::ContextForm { name, parent_id, status, notes };
    diesel::insert_into(contexts::table)
        .values(&new_context)
        .execute(conn)
}
