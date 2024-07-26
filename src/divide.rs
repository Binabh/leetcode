#[cfg(test)]
mod tests {
    fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        let mut result: i64 = 0;
        let mut multiplier = 1;
        if dividend > 0 && divisor < 0 {
            divisor = divisor * -1;
            multiplier = -1;
        }
        if dividend < 0 && divisor > 0 {
            dividend = dividend * -1;
            multiplier = -1;
        }
        if dividend < 0 && divisor < 0 {
            dividend = dividend * -1;
            divisor = divisor * -1;
        }

        while dividend >= divisor {
            dividend -= divisor;
            result += 1;
        }

        return (result * multiplier) as i32;
    }
    #[test]
    fn test() {
        assert_eq!(divide(4, 2), 2);
        assert_eq!(divide(10, 3), 3);
        assert_eq!(divide(7, -3), -2);
        assert_eq!(divide(-7, -3), 2);
        assert_eq!(divide(1, -1), -1);
        assert_eq!(divide(-2147483648, -1), 2147483647);
        assert_eq!(divide(-2147483648, 1), -2147483648);
        assert_eq!(divide(-2147483648, 2), -1073741824);
    }
}
