pub fn convert_to_base7(mut num: i32) -> String {
    let mut signing_value = 1;
    if num < 0 {
        num *= -1;
        signing_value *= -1;
    }
    let mut result = 0;
    let mut count = 0;
    while num > 0 {
        result += (num % 7) * (10i32.pow(count));
        num /= 7;
        count += 1;
    }
    (result * signing_value).to_string()
}

pub fn test() {
    assert_eq!(convert_to_base7(7), String::from("10"));
    assert_eq!(convert_to_base7(-7), String::from("-10"));
}

pub const NAME: &str = "convert_to_base7";
