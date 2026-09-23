#[macro_export]
macro_rules! schema {
    ($($name:ident : $ty:ident),* $(,)?) => {
        vec![$((stringify!($name).to_string(), ValueType::$ty)),*]
    };
}

#[macro_export]
macro_rules! entry {
    ($($name:ident : $val:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert(stringify!($name).to_string(), $crate::enums::Value::from($val));
        )*
        map
    }};
}

#[macro_export]
macro_rules! vec_val {
    ($($x:expr),* $(,)?) => {
        vec![$(Value::from($x)),*]
    };
}
