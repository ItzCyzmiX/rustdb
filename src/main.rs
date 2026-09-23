use rustdb::{
    db::DB,
    entry,
    enums::{Value, ValueType},
    schema, vec_val,
};

fn main() {
    let mut db = DB::new();

    let books_schema = schema! {
        id: Int,
        title: String,
        published: Bool,
        tags: Vec
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
            tags: vec_val!["database", "concepts"],
        })
        .unwrap();

    println!("{:?}", books.get_id(id))
}
