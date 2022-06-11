pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        if c == '(' {
            stack.push(')')
        } else if c == '{' {
            stack.push('}')
        } else if c == '[' {
            stack.push(']')
        } else if c
            == match stack.pop() {
                Some(c) => c,
                None => '_',
            }
        {
            continue;
        } else {
            return false;
        }
    }
    stack.is_empty()
}
pub fn test() {
    assert_eq!(is_valid(String::from("()")), true)
}

pub const NAME: &str = "is_valid";
