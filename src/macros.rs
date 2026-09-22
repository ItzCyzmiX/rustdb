#[macro_export]
macro_rules! schema {
    ($($name:ident : $ty:expr),* $(,)?) => {
        vec![$((stringify!($name).to_string(), $ty)),*]
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
