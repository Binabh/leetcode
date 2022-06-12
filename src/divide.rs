pub fn divide(mut dividend: i32, divisor: i32) -> i32 {
    let mut result = 0;
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    }
    if divisor == 1 {
        return dividend;
    }
    if divisor == -1 {
        return -dividend;
    }
    if dividend < 0 && divisor > 0 {
        while dividend < 0 {
            dividend += divisor;
            result += 1;
        }
        -(result - 1)
    } else if dividend > 0 && divisor < 0 {
        while dividend > 0 {
            dividend += divisor;
            result += 1;
        }
        -(result - 1)
    } else if dividend < 0 && divisor < 0 {
        while dividend < 0 {
            dividend -= divisor;
            result += 1;
        }
        result - 1
    } else {
        while dividend >= divisor {
            dividend -= divisor;
            result += 1;
        }
        result
    }
}

pub fn test() {
    assert_eq!(divide(4, 2), 2);
    assert_eq!(divide(10, 3), 3);
    assert_eq!(divide(7, -3), -2);
    assert_eq!(divide(-7, -3), 2);
    assert_eq!(divide(1, -1), -1);
    assert_eq!(divide(-2147483648, -1), 2147483647);
    assert_eq!(divide(-2147483648, 1), -2147483648);
    assert_eq!(divide(-2147483648, 2), -1073741824);
}

pub const NAME: &str = "divide";
