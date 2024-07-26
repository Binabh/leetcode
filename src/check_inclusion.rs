#[cfg(test)]
mod tests {
    fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut left_pointer: usize = 0;
        let mut right_pointer: usize = left_pointer + s1.len();
        let mut s1_count: [usize; 26] = [0; 26];
        let mut s2_count: [usize; 26] = [0; 26];
        for (idx, ch) in s1.chars().enumerate() {
            s1_count[ch as usize - 'a' as usize] += 1;
            s2_count[s2.chars().nth(idx).unwrap() as usize - 'a' as usize] += 1;
        }

        while right_pointer < s2.len() {
            if s1_count == s2_count {
                return true;
            }
            s2_count[s2.chars().nth(left_pointer).unwrap() as usize - 'a' as usize] -= 1;
            s2_count[s2.chars().nth(right_pointer).unwrap() as usize - 'a' as usize] += 1;
            right_pointer += 1;
            left_pointer += 1;
        }
        s1_count == s2_count
    }
    #[test]
    fn test() {
        assert_eq!(
            check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
        assert_eq!(
            check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }
}
