#[cfg(test)]
mod tests {
    fn letter_combinations(digits: String) -> Vec<String> {
        for _ in digits.chars() {}
        return vec![];
    }
    #[test]
    fn test() {
        assert_eq!(
            letter_combinations(String::from("")),
            vec![String::from(""); 0]
        )
    }
}
