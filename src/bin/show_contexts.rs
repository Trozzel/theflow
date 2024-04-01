use theflow::cli::insert;

fn main() {
    let stdin_args: Vec<String> = std::env::args().collect();
    assert!(stdin_args.len() > 4 && stdin_args.len() % 2 == 1);
    match insert(&stdin_args) {
        Ok(num_insert) => println!("Successfully inserted {} records", num_insert),
        Err(e) => panic!("Error inserting into table: {:?}", e)
    }
}
