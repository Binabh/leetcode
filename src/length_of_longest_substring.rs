#[cfg(test)]
mod tests {
    fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }
        let mut res: i32 = 0;
        let mut left_pointer: usize = 0;
        let mut right_pointer: usize = left_pointer + 1;
        let mut current_length = 1;
        while right_pointer <= (s.len() - 1) {
            if s[left_pointer..right_pointer].contains(s.as_bytes()[right_pointer] as char) {
                left_pointer += 1;
                right_pointer = left_pointer + 1;
                current_length = 1;
            } else {
                current_length += 1;
                right_pointer += 1;
            }
            if current_length > res {
                res = current_length;
            }
        }

        res
    }

    #[test]
    fn test() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("aa".to_string()), 1);
        assert_eq!(length_of_longest_substring("ab".to_string()), 2);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
        assert_eq!(length_of_longest_substring("a".to_string()), 1);
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    }
}
