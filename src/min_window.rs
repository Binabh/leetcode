pub fn min_window(s: String, t: String) -> String {
    if t.len() > s.len() || t == "".to_string() {
        return "".to_string();
    }
    let mut res: [usize; 2] = [0; 2];
    let mut res_size = usize::MAX;
    let mut has: usize = 0;
    let mut need: usize = 0;
    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = left_pointer;
    let mut t_count: [usize; 58] = [0; 58];
    let mut s_count: [usize; 58] = [0; 58];
    for (_, ch) in t.chars().enumerate() {
        t_count[ch as usize - 'A' as usize] += 1;
        need += 1;
    }
    while right_pointer < s.len() {
        if t.contains(s.chars().nth(right_pointer).unwrap()) {
            s_count[s.chars().nth(right_pointer).unwrap() as usize - 'A' as usize] += 1;
            has += 1;
        }
        // if t.contains(s.chars().nth(right_pointer).unwrap())
        //     && (s_count[s.chars().nth(right_pointer).unwrap() as usize - 'A' as usize]
        //         < t_count[s.chars().nth(right_pointer).unwrap() as usize - 'A' as usize])
        // {
        //     s_count[s.chars().nth(right_pointer).unwrap() as usize - 'A' as usize] += 1;
        //     has += 1;
        // }
        print!("{},{}\n", has, need);
        while has == need {
            if ((right_pointer + 1) - left_pointer) < res_size {
                res = [left_pointer, right_pointer + 1];
                res_size = (right_pointer + 1) - left_pointer;
            }
            if s_count[s.chars().nth(left_pointer).unwrap() as usize - 'A' as usize] > 0 {
                s_count[s.chars().nth(left_pointer).unwrap() as usize - 'A' as usize] -= 1;
                has -= 1;
            }
            left_pointer += 1;
        }
        right_pointer += 1;
    }
    if res_size != usize::MAX {
        return s[res[0]..res[1]].to_string();
    } else {
        return "".to_string();
    }
}

pub fn test() {
    assert_eq!(
        min_window("a".to_string(), "a".to_string()),
        "a".to_string()
    );
    assert_eq!(
        min_window("Az".to_string(), "z".to_string()),
        "z".to_string()
    );
    assert_eq!(
        min_window("ABCD".to_string(), "ABC".to_string()),
        "ABC".to_string()
    );
    assert_eq!(
        min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC".to_string()
    );
    assert_eq!(
        min_window("bba".to_string(), "ab".to_string()),
        "ba".to_string()
    );
}

pub const NAME: &str = "min_window";
