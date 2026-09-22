use rustdb::{db::DB, entry, enums::ValueType, schema, types::TableEntry};

fn main() {
    let mut mydb = DB::new();

    let schema = schema! {
        name: ValueType::String,
        hp: ValueType::Int,
        items: ValueType::Vec
    };

    match mydb.new_table("players", schema) {
        Ok(_) => println!("{:?}", "created table succesfully"),
        Err(_) => println!("No Players table found"),
    }

    let players_table = mydb.from("players").unwrap();

    let player: TableEntry = entry! {
        name: "CyzmiX",
        hp: 10
    };

    match players_table.add(&player) {
        Ok(_) => println!("Added new player!"),
        Err(err) => println!("{:?}", err),
    };

    println!("{:?}", players_table.remove_exact(&player));

    println!("{:?}", players_table.get_exact(&player));
}
