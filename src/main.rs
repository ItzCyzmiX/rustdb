use rustdb::{
    db::DB,
    entry,
    enums::{Value, ValueType},
    schema,
    types::TableEntry,
    vec_val,
};

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
        hp: 10,
        items: vec_val!["sword", "shield"]
    };

    match players_table.add(&player) {
        Ok(_) => println!("Added new player!"),
        Err(err) => println!("{:?}", err),
    };

    players_table.update_if(
        |p| p["name"] == Value::String("CyzmiX".to_string()),
        &entry! {
            name: "ItzCyzmiX",
        },
    );

    println!("{:?}", players_table.remove_exact(&player));

    println!(
        "{:?}",
        players_table.get_if(|f| f["name"] == Value::String("ItzCyzmiX".to_string()))
    );
}
