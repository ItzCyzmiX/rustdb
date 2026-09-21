use std::collections::HashMap;

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

impl Operator {
    fn compare(&self, a: &Value, b: &Value) -> bool {
        match self {
            Operator::Bigger => a > b,
            Operator::Smaller => a < b,
            Operator::BiggerOrEq => a >= b,
            Operator::SmallerOrEq => a <= b,
            Operator::Eq => a == b,
            Operator::NotEq => a != b,
        }
    }
}

#[derive(Debug)]
struct DB {
    data: HashMap<String, Value>,
}

impl DB {
    fn set(&mut self, name: &str, data: Value) {
        self.data.insert(String::from(name), data);
    }

    fn get(&self, name: &str) -> Option<&Value> {
        self.data.get(name)
    }

    fn get_if<F>(&self, filter_method: F) -> HashMap<&String, &Value>
    where
        F: Fn(&Value) -> bool,
    {
        self.data.iter().filter(|f| filter_method(f.1)).collect()
    }

    fn get_with_op(&self, op: Operator, value: Value) -> HashMap<&String, &Value> {
        self.data
            .iter()
            .filter(|f| op.compare(f.1, &value))
            .collect()
    }
}

fn main() {
    let mut mydb = DB {
        data: HashMap::new(),
    };

    mydb.set("hello", Value::Bool(true));
    mydb.set("hi", Value::String(String::from("20")));
    mydb.set("nihai", Value::Int(5));

    println!("{:?}", mydb.get_if(|e| e.eq(&Value::Bool(true))).keys())
}
