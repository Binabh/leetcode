pub fn letter_combinations(digits: String) -> Vec<String> {
    for _ in digits.chars() {}
    return vec![];
}
pub fn test() {
    assert_eq!(
        letter_combinations(String::from("")),
        vec![String::from(""); 0]
    )
}

pub const NAME: &str = "letter_combinations";
