pub mod models;
pub mod schema;

//use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

/// Establish and return the database connection 
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Functiont to save new context to contexts table
use self::models::{Context, NewContext};

pub fn create_context (conn: &mut MysqlConnection,
                    name: &str,
                    parent_id: Option<i64>,
                    status: &str) -> Context {
    
    use theflow::schema::contexts;

    let new_context = NewContext { name, parent_id, status };
    diesel::insert_into(context::table)
        .values(&new_context)
        .returning(Context::as_returning())
        .get_result(conn)
        .expect("Error saving new context");
}
