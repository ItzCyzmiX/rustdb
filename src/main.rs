use rustdb::{
    db::DB,
    enums::{Operator, Value, ValueType},
};
use std::{collections::HashMap, vec};

fn main() {
    let mut mydb = DB::new();

    let schema = vec![
        ("hp".to_string(), ValueType::Int),
        ("name".to_string(), ValueType::String),
    ];

    match mydb.new_table("players", schema) {
        Ok(_) => println!("{:?}", "created table succesfully"),
        Err(_) => println!("Oh no"),
    }

    match mydb.from("players").unwrap().add(HashMap::from([
        ("hp".to_string(), Value::Int(10)),
        ("name".to_string(), Value::String("Hello".to_string())),
    ])) {
        Ok(_) => println!("Added"),
        Err(err) => println!("{:?}", err),
    };

    println!(
        "{:?}",
        mydb.from("players").unwrap().remove(HashMap::from([
            ("hp".to_string(), Value::Int(10)),
            ("name".to_string(), Value::String("Hello".to_string())),
        ]))
    );

    println!(
        "{:?}",
        mydb.from("players")
            .unwrap()
            .get_with_op("hp", Operator::Bigger, Value::Int(5))
    );
}
