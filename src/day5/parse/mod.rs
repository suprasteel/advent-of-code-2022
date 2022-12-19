pub mod instruction;
pub mod ship;

pub fn skip_whitespace(input: &str) -> &str {
    match input.find(|c: char| !c.is_whitespace()) {
        Some(index) => &input[index..],
        None => "",
    }
}
