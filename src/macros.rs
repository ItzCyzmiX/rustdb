#[macro_export]
macro_rules! constraint {
    (min($v:expr)) => {
        $crate::enums::Constraints::Min($v)
    };
    (max($v:expr)) => {
        $crate::enums::Constraints::Max($v)
    };
    (min_f($v:expr)) => {
        $crate::enums::Constraints::MinF($v)
    };
    (max_f($v:expr)) => {
        $crate::enums::Constraints::MaxF($v)
    };
    (between($v:expr, $m:expr)) => {
        $crate::enums::Constraints::Between($v, $m)
    };
    (contains($v:expr)) => {
        $crate::enums::Constraints::Contains($v.to_string())
    };
    (not_contains($v:expr)) => {
        $crate::enums::Constraints::NotContains($v.to_string())
    };
    (starts_with($v:expr)) => {
        $crate::enums::Constraints::StartsWith($v.to_string())
    };
    (not_starts_with($v:expr)) => {
        $crate::enums::Constraints::NotStartsWith($v.to_string())
    };
    (unique) => {
        $crate::enums::Constraints::Unique
    };
}

#[macro_export]
macro_rules! schema {
    (
        $(
            $name:ident : $ty:ident
            $( => [ $( $c:ident $( ( $($arg:tt)* ) )? ),* $(,)? ] )?
        ),* $(,)?
    ) => {
        vec![$((
            stringify!($name).to_string(),
            $crate::enums::ValueType::$ty,
            {
                let constraints: Vec<$crate::enums::Constraints> = vec![
                    $( $( $crate::constraint!($c $( ( $($arg)* ) )?) ),* )?
                ];
                constraints
            }
        )),*]
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
