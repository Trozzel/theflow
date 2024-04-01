// CONSTANT TABLE AND COLUMN NAMES
/******************************************************************************/
pub const TABLES: &[&str] = &["contexts", "folders", "projects", "tasks"];

pub const CRUD_OPS: &[&str] = &["insert", "update", "insert", "delete"];

pub const CONTEXT_FOLDER_COLS: &[&str] = &["name", "parent_id", "status", "notes"];
pub const PROJECT_COLS: &[&str] = &["name", "parent_id", "status", "notes",
    "context_id", "folder_id", "flagged", "deferred", "due", "is_repeating",
    "repeat_schedule", "complete_with_last", "review_schedule", "project_type"];
pub const TASK_COLS: &[&str] = &["name", "parent_id", "status", "notes",
    "context_id", "project_id", "flagged", "deferred", "due", "is_repeating",
    "repeat_schedule", "complete_with_last", "complete_with_last", 
    "review_schedule", "project_type"];

/// Verify table name has given column
///
/// Since you cannot have a static HashMap, we must use the static arrays and 
/// perform logic on them
/******************************************************************************/
pub fn table_col_mapper<'a>(table: &'a str, colname: &'a str) -> Result<(), String> {
    // 1. Check to see if table exists
    if !TABLES.contains(&table) {
        Err(format!("Table, '{}', does not exist", table))
    }
    // 2. Check to see if table contains given column
    else {
        match table {
            "contexts" => {
                if !CONTEXT_FOLDER_COLS.contains(&colname) {
                    Err(format!("Table, '{}', does not have column, '{}'", table, colname))
                }
                else {
                    Ok(())
                }
            }
            "folders" => {
                if !CONTEXT_FOLDER_COLS.contains(&colname) {
                    Err(format!("Table, '{}', does not have column, '{}'", table, colname))
                }
                else {
                    Ok(())
                }
            }
            "projects" => {
                if !PROJECT_COLS.contains(&colname) {
                    Err(format!("Table, '{}', does not have column, '{}'", table, colname))
                }
                else {
                    Ok(())
                }
            }
            "tasks" => {
                if !TASK_COLS.contains(&colname) {
                    Err(format!("Table, '{}', does not have column, '{}'", table, colname))
                }
                else {
                    Ok(())
                }
            }
            _ => Err(String::from("It should be impossible to get here"))
        }
    }
}
