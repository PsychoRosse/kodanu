/// Creates a `HashMap` with the given capacity and inserts the given key-value pairs.
///
/// The `cap` argument reserves capacity up front, the rest of the syntax is a
/// comma-separated list of `key => value` pairs.
///
/// `HashMap` is resolved at the call site, so the caller must have
/// `std::collections::HashMap` in scope.
///
/// # Examples
///
/// ```
/// use {kodanu_macros::map_with_capacity, std::collections::HashMap};
///
/// let map: HashMap<&str, i32> = map_with_capacity!(2; "a" => 1, "b" => 2);
///
/// assert_eq!(map.len(), 2);
/// assert_eq!(map["b"], 2);
/// ```
#[macro_export]
macro_rules! map_with_capacity {
    ($cap:expr; $($key:expr => $value:expr),* $(,)?) => {{
        let mut map = HashMap::with_capacity($cap);

        $(
            map.insert($key, $value);
        )* map
    }};
}
