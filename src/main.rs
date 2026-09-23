use rustdb::{
    db::DB,
    entry,
    enums::{Value, ValueType},
    schema, vec_val,
};

fn main() {
    let mut db = DB::new();

    let books_schema = schema! {
        id: ValueType::Int,
        title: ValueType::String,
        published: ValueType::Bool,
        tags: ValueType::Vec
    };

    db.new_table("books", books_schema).unwrap();

    let books = db.from("books").unwrap();

    books
        .add(&entry! {
            id: 1,
            title: "Rust in Practice",
            published: true,
            tags: vec_val!["rust", "systems", "learning"]
        })
        .unwrap();

    let id = books
        .add(&entry! {
            id: 2,
            title: "Database Design 101",
            published: true,
            tags: vec_val!["database", "concepts"]
        })
        .unwrap();

    println!("{:?}", books.remove_id(id));
    println!("{:?}", books.get_id(id))

    // let available = books.get_if(|row| row["published"] == Value::Bool(true));
    // println!("Published books: {available:?}");

    // books.update_if(
    //     |row| row["id"] == Value::Int(2),
    //     &entry! {
    //         title: "Database Design for Builders",
    //         published: true,
    //     },
    // );

    // let removed =
    //     books.remove_if(|row| row["title"] == Value::String("Rust in Practice".to_string()));
    // println!("Removed: {removed:?}");
}
