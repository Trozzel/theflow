use std::collections::HashMap;
use diesel::QueryResult;
use crate::establish_connection;
use crate::crudops::insert_context;

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
    pub fn new(table: &'a str,
               crud_op: &'a str,
               col_to_vals: HashMap<String, String>) -> TableCrudCli<'a> 
    {
        TableCrudCli{ table, crud_op, col_to_vals }
    }

    pub fn display(&self) {
        println!("Your command: {} {} {:?}", &self.table, &self.crud_op, &self.col_to_vals);
    }
}

// MAIN CLI
/******************************************************************************/
pub fn get_table_crud_cli<'a>(args: &'a Vec<String>)  -> TableCrudCli {
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
        },
        _ => {
            panic!("'{}' is not a valid table", table);
        },
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
        }
        else {
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
pub fn insert<'a>(args: &'a Vec<String>) -> QueryResult<usize> {
    let table: &str = &args[1];
    assert!(TABLES.contains(&table));

    let crud_op: &str = &args[2];
    assert!(CRUD_OPS.contains(&crud_op));
    
    assert!(args[3].starts_with("--") && args[3].ends_with("name"));
    let name: &str = &args[3][2..];

    insert_context(&mut establish_connection(), name, None, None, None)
}
