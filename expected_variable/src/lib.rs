pub use case;
pub use case::CaseExt;
pub use edit_distance::edit_distance;

pub fn expected_variable(target_str: &str, expected_str: &str) -> Option<String> {
    let target: String = target_str.to_lowercase();
    let expected: String = expected_str.to_lowercase();

    if (!target.is_camel_lowercase() && target_str.to_camel().to_snake() != target) ||
        target_str.contains(['-', ' ']) || target_str == "" || expected_str == ""{
        return  None;
    }

    let differ_chars = edit_distance(&target, &expected);
    let alikeness = 1.0 - differ_chars as f64 / expected.len() as f64;
    let alikeness = (alikeness * 100.0).round();

    if alikeness > 50.0{
        return Some(format!("{alikeness}%"));
    }

    return None;
}