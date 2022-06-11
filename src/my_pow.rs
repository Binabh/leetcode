pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
    let mut result = 1.0;
    let mut sign_multiplier = 1.0;
    if n < 0 {
        x = 1.0 / x;
        n = n * -1
    }
    if x < 0.0 {
        x *= -1.0;
        if n % 2 != 0 {
            sign_multiplier *= -1.0
        }
    }
    let mut prev_result = 1.0;
    while n != 0 {
        result *= x;
        let result_round = (result * 10000000000000.0).round() / 10000000000000.0;
        if prev_result == result_round {
            break;
        }
        prev_result = result_round;
        n -= 1;
    }
    ((prev_result * 100000.0).round() / 100000.0) * sign_multiplier
}
pub fn test() {
    assert_eq!(my_pow(2.0, 2), 4.0)
}

pub const NAME: &str = "my_pow";
