#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let map: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut prev = 'I';
        for letter in s.chars().rev() {
            let val: i32 = match map.get(&letter) {
                Some(val) => *val,
                None => 0,
            };
            let prev_val: i32 = match map.get(&prev) {
                Some(val) => *val,
                None => 0,
            };
            if val < prev_val {
                result -= val
            } else {
                result += val
            }
            prev = letter;
        }
        return result;
    }
    #[test]
    fn test() {
        assert_eq!(roman_to_int(String::from("III")), 3)
    }
}
