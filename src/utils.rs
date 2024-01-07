pub fn exstract_head_int(s: &str) -> (Option<i64>, Option<&str>) {
    let s = s.trim();

    let no_number_idx = s.find(|c: char| !c.is_numeric()).unwrap_or(s.len());
    let (num, rest) = s.split_at(no_number_idx);

    if num.is_empty() && rest.is_empty() {
        (None, None)
    } else if num.is_empty() {
        (None, Some(rest))
    } else if rest.is_empty() {
        (Some(num.parse::<_>().unwrap()), None)
    } else {
        (Some(num.parse::<_>().unwrap()), Some(rest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exstract_head_int() {
        assert_eq!(exstract_head_int(" 42 "), (Some(42), None));
        assert_eq!(exstract_head_int("42 +"), (Some(42), Some(" +")));
        assert_eq!(exstract_head_int("+ 42"), (None, Some("+ 42")));
        assert_eq!(exstract_head_int("+ 42 +"), (None, Some("+ 42 +")));
        assert_eq!(exstract_head_int(""), (None, None));
    }
}
