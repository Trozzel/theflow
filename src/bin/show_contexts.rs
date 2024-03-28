use self::models::*;
use diesel::prelude::*;
use theflow::*;

fn main() {
    // lets us access posts directly
    use self::schema::contexts::dsl::*;

    let connection = &mut establish_connection();
    let results = contexts
        .filter(name.eq("George"))
        .limit(5)
        .select(Context::as_select())
        .load(connection)
        .expect("Error loading contexts");

    println!("Displaying {} contexts", results.len());
    for context in results {
        println!("{}", context.name);
        println!("----------");
    }
}
