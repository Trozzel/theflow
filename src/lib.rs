// ROOT
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

pub mod cli;
pub mod models;
pub mod schema;
pub mod helpers;
pub mod crudops;

//use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

/// Establish and return the database connection
/******************************************************************************/
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

