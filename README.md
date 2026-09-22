# RustDB

A small work-in-progress database project written in Rust.

## Status

This project is still under active development. It is intended as a lightweight learning and experimentation database focused on table storage, row handling, and simple typed values.

## Features

- Rust-based database implementation
- Table and record handling
- Basic schema validation for rows
- Simple value types (`Int`, `Float`, `String`, `Bool`, `Vec`)
- Helper macros for building schemas and rows

## Example Usage

```rust
use rustdb::{
    db::DB,
    entry,
    enums::{Value, ValueType},
    schema,
    vec_val,
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

    books.add(&entry! {
        id: 1,
        title: "Rust in Practice",
        published: true,
        tags: vec_val!["rust", "systems", "learning"]
    }).unwrap();

    books.add(&entry! {
        id: 2,
        title: "Database Design 101",
        published: false,
        tags: vec_val!["database", "concepts"]
    }).unwrap();

    let available = books.get_if(|row| row["published"] == Value::Bool(true));
    println!("Published books: {available:?}");

    books.update_if(
        |row| row["id"] == Value::Int(2),
        &entry! {
            title: "Database Design for Builders",
            published: true,
        },
    );

    let removed = books.remove_if(|row| row["title"] == Value::String("Rust in Practice".to_string()));
    println!("Removed: {removed:?}");
}
```

## Notes

This API is intentionally small and evolving. The current implementation is best treated as an educational database prototype rather than a production-ready storage engine.
