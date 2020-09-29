/// Fetch a sanitized version of a type's internal name
pub fn type_name<T>() -> String
where
    T: ?Sized,
{
    strip_crate_names(std::any::type_name::<T>())
}

/// Strip namespaces out of type names provided by std::any::type_name
pub fn strip_crate_names(string: &str) -> String {
    let before: &str;
    let after: Option<&str>;

    if let Some(open_bracket) = string.find('<') {
        let (split_before, split_after) = string.split_at(open_bracket);
        before = split_before;
        after = Some(split_after);
    } else {
        before = string;
        after = None;
    }

    let before = before.split("::").last().unwrap();
    if let Some(after) = after {
        before.to_string() + "<" + &strip_crate_names(&after[1..after.len() - 1]) + ">"
    } else {
        before.into()
    }
}
