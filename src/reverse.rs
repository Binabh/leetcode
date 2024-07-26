#[cfg(test)]
mod tests {

    fn reverse(mut x: i32) -> i32 {
        let mut sign_multiplier = 1;
        if x < 0 {
            x *= -1;
            sign_multiplier *= -1
        }
        let mut y: i32 = 0;
        while x > 0 {
            y = match y.checked_mul(10) {
                Some(n) => n,
                None => return 0,
            } + x % 10;
            x = x / 10;
        }
        return y * sign_multiplier;
    }
    #[test]
    fn test() {
        assert_eq!(reverse(123), 321)
    }
}
