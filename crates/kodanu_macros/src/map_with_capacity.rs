#[macro_export]
macro_rules! map_with_capacity {
    ($cap:expr; $($key:expr => $value:expr),* $(,)?) => {{
        let mut map = HashMap::with_capacity($cap);

        $(
            map.insert($key, $value);
        )* map
    }};
}
