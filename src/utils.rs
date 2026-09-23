use std::{collections::HashMap, hash::Hash};

pub fn maps_match<T, U>(map1: &HashMap<T, U>, map2: &HashMap<T, U>) -> bool
where
    T: Eq + Hash,
    U: PartialEq,
{
    map1.len() == map2.len() && map1.iter().all(|(key, value)| map2.get(key) == Some(value))
}

pub fn keys_match<T: Eq + Hash, U, V>(
    map1: &HashMap<String, U>,
    map2: &HashMap<String, V>,
) -> bool {
    map1.len() == map2.len()
        && map1
            .keys()
            .all(|k| k == &"ID".to_string() || map2.contains_key(k))
}
