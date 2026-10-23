use convert_case::ccase;
use edit_distance::edit_distance;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let is_camel = compared.eq_ignore_ascii_case(&ccase!(camel, compared));
    let is_snake = compared.eq_ignore_ascii_case(&ccase!(snake, compared));

    if !is_camel && !is_snake {
        return None;
    }

    let distance = edit_distance(compared.to_lowercase(), expected.to_lowercase());

    let similarity = (1.0 - distance as f64 / expected.chars().count() as f64) * 100.0;

    if similarity > 50.0 {
        Some(format!("{:.0}%", similarity))
    } else {
        None
    }
}
