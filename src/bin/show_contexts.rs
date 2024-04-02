use theflow::cli::insert;

fn main() {
    let stdin_args: Vec<String> = std::env::args().collect();
    match insert(&stdin_args) {
        Ok(num_insert) => println!("Successfully inserted {} records", 
                                   num_insert.expect("Failed to insert")),
        Err(e) => panic!("Error inserting into table: {:?}", e)
    }
}
