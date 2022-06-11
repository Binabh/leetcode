pub fn my_sqrt(n: i32) -> i32 {
    let num: u32 = n as u32;
    let mut x: u32 = num.to_owned();
    let mut y: u32 = 1;
    while x > y {
        x = (x + y) / 2;
        y = num / x;
    }

    return x as i32;
}
pub fn test() {
    assert_eq!(my_sqrt(4), 2)
}

pub const NAME: &str = "my_sqrt";
