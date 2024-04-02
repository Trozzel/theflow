use crate::crudops::insert_context;
use crate::establish_connection;
use diesel::QueryResult;
use std::collections::HashMap;

use crate::helpers::*;

// TableCrudCli
/******************************************************************************/
#[derive(Debug)]
pub struct TableCrudCli<'a> {
    table: &'a str,
    crud_op: &'a str,
    col_to_vals: HashMap<String, String>,
}

// Implementation methods
impl<'a> TableCrudCli<'a> {
    pub fn new(
        table: &'a str,
        crud_op: &'a str,
        col_to_vals: HashMap<String, String>,
    ) -> TableCrudCli<'a> {
        TableCrudCli {
            table,
            crud_op,
            col_to_vals,
        }
    }

    pub fn display(&self) {
        println!(
            "Your command: {} {} {:?}",
            &self.table, &self.crud_op, &self.col_to_vals
        );
    }
}

// MAIN CLI
/******************************************************************************/
pub fn get_table_crud_cli<'a>(args: &'a Vec<String>) -> TableCrudCli {
    // 1. Verify table name
    let table: &str = &args[1];
    if !TABLES.contains(&table) {
        panic!("'{}', is not a valid table", table);
    }

    // 2. Verify CRUD operation
    let crud_op: &str = &args[2];
    if !CRUD_OPS.contains(&crud_op) {
        panic!("'{}', is not a valid table", table);
    }

    // Pointer to the --<column> <value> fields
    let col_to_vals = &args[3..];

    // INSERT should have same amount of values for columns
    let col_val_map = match crud_op {
        "insert" => {
            assert!(col_to_vals.len() % 2 == 0);
            get_hashmap(&col_to_vals)
        }
        _ => {
            panic!("'{}' is not a valid table", table);
        }
    };

    TableCrudCli::new(table, crud_op, col_val_map)
}

// INSERT FUNCTIONALITY
/******************************************************************************/
fn get_hashmap<'a>(cols_vals: &'a [String]) -> HashMap<String, String> {
    // Initiate colums and values
    let mut cols = vec![];
    let mut vals = vec![];

    let mut is_column = true;
    for arg in cols_vals {
        if is_column {
            assert!(arg.starts_with("--"));
            is_column = false;
            cols.push(&arg[2..]);
        } else {
            is_column = true;
            vals.push(&arg[..]);
        }
    }

    let mut col_val_map: HashMap<String, String> = HashMap::new();
    for (i, col) in cols.iter().enumerate() {
        col_val_map.insert((*col).to_string(), (*vals[i]).to_string());
    }
    col_val_map
}

// INSERT A CONTEXT
pub fn insert<'a>(args: &'a Vec<String>) -> Result<QueryResult<usize>, String> {
    // Verify proper amount of arguments
    if args.len() < 5 && args.len() % 2 == 0 {
        return Err(String::from("Improper amount of arguments"));
    }

    // Verify valid table name
    let table: &str = &args[1];
    if !TABLES.contains(&table) {
        return Err(format!("Invalid table, '{}'", table));
    }

    // Verify valid CRUD operation
    let crud_op: &str = &args[2];
    if !CRUD_OPS.contains(&crud_op) {
        return Err(format!("Invalid CRUD operation, '{}'", crud_op));
    }

    // Verify first column name flag is --name
    if !args[3].starts_with("--") && !args[3].ends_with("name") {
        return Err(String::from("First flagged item must be '--name'"));
    }

    // Set parent_id, if present
    let pid_idx = args.iter().position(|x| x == "--parent_id");
    let parent_id: Option<&str> = match pid_idx {
        Some(idx) => Some(&args[idx + 1][..]),
        None => None,
    };

    let parent_id = match parent_id {
        Some(numstr) => Some(numstr.parse::<i64>().expect("Not valid integer string")),
        _ => None
    };

    // Set status, if present
    let status_idx = args.iter().position(|x| x == "--status");
    let status: Option<&str> = match status_idx {
        Some(idx) => Some(&args[idx + 1][..]),
        None => None,
    };

    // Read name
    let name: &str = &args[4][..];

    Ok(insert_context(
        &mut establish_connection(),
        name,
        parent_id,
        status,
        None,
    ))
}
