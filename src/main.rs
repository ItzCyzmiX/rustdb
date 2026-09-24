use rustdb::{
    db::DB,
    entry,
    enums::{DBError, Null::Null, Value},
    schema, vec_val,
};

fn main() {
    let mut db = DB::new();

    db.new_table(
        "books",
        schema! {
            id: Int => [between(10, 50)],
            title: String => [unique],
            published: Bool,
            tags: OptionalVec,

        },
    )
    .unwrap();

    let books = db.from("books").unwrap();

    let first_id = books
        .add(&entry! {
            id: 40,
            title: "Rust in Practice",
            published: true,
            tags: Null,
        })
        .unwrap();

    let second_id = books
        .add(&entry! {
            id: 30,
            title: "Database Design 101",
            published: true,
            tags: vec_val!["database"]
        })
        .unwrap();

    println!("Inserted books: {first_id}, {second_id}");
    println!("First book: {:?}", books.get_id(first_id));

    let published_books = books.get_if(|row| row.get("published") == Some(&Value::Bool(true)));
    println!("Published books: {published_books:?}");

    books
        .update_id(
            first_id,
            &entry! {
                title: "Rust in Practice, Second Edition"
            },
        )
        .unwrap();
    println!("Updated first book: {:?}", books.get_id(first_id));

    let duplicate = books.add(&entry! {
        id: 20,
        title: "Database Design 101",
        published: false,
        tags: vec_val!["database"]
    });

    match duplicate {
        Err(DBError::ConstraintNotMet) => println!("Duplicate title rejected"),
        other => panic!("expected ConstraintNotMet, got {other:?}"),
    }

    let removed = books.remove_id(second_id).unwrap();
    println!("Removed book: {removed:?}");
}
