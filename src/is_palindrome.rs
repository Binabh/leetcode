pub fn is_palindrome(x: i32) -> bool {
    let mut y = 0;
    let mut xcopy = x;
    while xcopy > 0 {
        y = y * 10 + xcopy % 10;
        xcopy = xcopy / 10;
    }
    return x == y;
}
pub fn test() {
    assert_eq!(is_palindrome(121), true)
}

pub const NAME: &str = "is_palindrome";
