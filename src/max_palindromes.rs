pub fn max_palindromes(s: String, k: i32) -> i32 {
    let mut result = 0;
    let s_bytes = s.as_bytes();
    for b in s_bytes {
        
    }

    result
}

pub fn test() {
    assert_eq!(max_palindromes("adbcda".to_string(), 2), 0);
    assert_eq!(max_palindromes("abaccdbbd".to_string(), 3), 2);
    assert_eq!(max_palindromes("zxxyxxz".to_string(), 3), 1);
    assert_eq!(max_palindromes("abaccdbbdtzxxzt".to_string(), 3), 3);
    assert_eq!(max_palindromes("abaccdbbdtzxzt".to_string(), 3), 3);
}

pub const NAME: &str = "max_palindromes";
