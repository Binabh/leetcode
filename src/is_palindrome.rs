#[cfg(test)]
mod tests {
    fn is_palindrome(x: i32) -> bool {
        let mut y = 0;
        let mut xcopy = x;
        while xcopy > 0 {
            y = y * 10 + xcopy % 10;
            xcopy = xcopy / 10;
        }
        return x == y;
    }
    fn is_palindrome_string(s: String) -> bool {
        let mut left = 0;
        let mut right = s.len() - 1;
        let s_lower = s.to_lowercase();
        let s_chars = s_lower.as_bytes();
        while left < right {
            let left_char = s_chars[left];
            let right_char = s_chars[right];
            if (left_char < 97 || left_char > 122) && (left_char < 48 || left_char > 57) {
                left += 1;
                continue;
            }
            if (right_char < 97 || right_char > 122) && (right_char < 48 || right_char > 57) {
                right -= 1;
                continue;
            }
            if left_char != right_char {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
    #[test]
    fn test() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome_string("Azbza".to_string()), true);
        assert_eq!(
            is_palindrome_string("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(is_palindrome_string("race a car".to_string()), false);
        assert_eq!(is_palindrome_string("0P".to_string()), false);
        assert_eq!(is_palindrome_string("0P0".to_string()), true);
    }
}
