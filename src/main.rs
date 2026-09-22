use std::{collections::HashMap, vec};

#[derive(Debug)]
enum ValueType {
    Int,
    Float,
    String,
    Bool,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Bigger,
    Smaller,
    Eq,
    BiggerOrEq,
    SmallerOrEq,
    NotEq,
}

// impl Operator {
//     fn compare(&self, a: &HashMap<String, Value>, b: &HashMap<&String, &Value>) -> bool {
//         match self {
//             Operator::Bigger => a > b,
//             Operator::Smaller => a < b,
//             Operator::BiggerOrEq => a >= b,
//             Operator::SmallerOrEq => a <= b,
//             Operator::Eq => a == b,
//             Operator::NotEq => a != b,
//         }
//     }
// }

#[derive(Debug, PartialEq)]
struct Table {
    rows: Vec<HashMap<String, Value>>,
    schema: HashMap<String, ValueType>,
    name: String,
}

#[derive(Debug)]
struct DB {
    //data: HashMap<String, Value>,
    tables: HashMap<String, Table>,
}

impl DB {
    pub fn new() -> Self {
        DB {
            tables: HashMap::new(),
        }
    }

    fn new_table(&mut self, name: &str, schema: Vec<(String, ValueType)>) -> Result<bool, bool> {
        let t = Table {
            rows: Vec::new(),
            schema: HashMap::from_iter(schema),
            name: name.to_string(),
        };
        if self.tables.contains_key(name) {
            return Err(false);
        }
        return Ok(None == self.tables.insert(String::from(name), t));
    }

    fn from(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.get_mut(name)
    }
}

impl Table {
    fn add(&mut self, data: Vec<(String, Value)>) {
        self.rows.push(HashMap::from_iter(data));
    }

    fn get(&mut self, value: Vec<(String, Value)>) -> Option<&mut HashMap<String, Value>> {
        let target = HashMap::from_iter(value);

        self.rows.iter_mut().find(|row| **row == target)
    }

    fn get_if<F>(&self, filter_method: F) -> Vec<&HashMap<String, Value>>
    where
        F: Fn(&HashMap<String, Value>) -> bool,
    {
        self.rows.iter().filter(|f| filter_method(*f)).collect()
    }

    // fn get_if<F>(&self, filter_method: F) -> HashMap<&String, &Value>
    // where
    //     F: Fn(&HashMap<String, Value>) -> bool,
    // {
    //     self.rows.iter().filter(|f| filter_method(f.1)).collect()
    // }

    // fn get_with_op(&self, op: Operator, value: Value) -> HashMap<&String, &Value> {
    //     self.rows
    //         .iter()
    //         .filter(|f| op.compare(f.1, &value))
    //         .collect()
    // }
}

fn main() {
    let mut mydb = DB::new();

    match mydb.new_table("players", vec![("hp".to_string(), ValueType::Int)]) {
        Ok(_) => println!("{:?}", "created table succesfully"),
        Err(_) => println!("Oh no"),
    }

    mydb.from("players")
        .unwrap()
        .add(vec![("hp".to_string(), Value::Int(100))]);

    println!(
        "{:?}",
        mydb.from("players")
            .unwrap()
            .get_if(|item| item["hp"] > Value::Int(10))
    );
}
