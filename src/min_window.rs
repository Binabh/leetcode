pub fn min_window(s: String, t: String) -> String {
    if t.len() > s.len() || t == "".to_string() {
        return "".to_string();
    }
    let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());
    let mut res: [usize; 2] = [0; 2];
    let mut res_size = s.len() + 1;
    let mut has: usize = 0;
    let need: usize = t.len();
    let mut left_pointer: usize = 0;
    let mut t_count: [usize; 58] = [0; 58];
    let mut s_count: [usize; 58] = [0; 58];
    let base = 'A' as usize;
    for b in t_bytes {
        t_count[*b as usize - base] += 1;
    }
    for (i, b) in s_bytes.into_iter().enumerate() {
        if t_count[*b as usize - base] > 0 {
            s_count[*b as usize - base] += 1;
            if s_count[*b as usize - base] <= t_count[*b as usize - base] {
                has += 1;
            }
        }
        while has == need {
            if i - left_pointer < res_size {
                res = [left_pointer, i];
                res_size = i - left_pointer;
            }
            if t_count[s_bytes[left_pointer] as usize - base] > 0 {
                s_count[s_bytes[left_pointer] as usize - base] -= 1;
                if s_count[s_bytes[left_pointer] as usize - base]
                    < t_count[s_bytes[left_pointer] as usize - base]
                {
                    has -= 1;
                }
            }
            left_pointer += 1;
        }
    }
    if res_size != s.len() + 1 {
        return s[res[0]..res[1] + 1].to_string();
    } else {
        return "".to_string();
    }
}

pub fn test() {
    assert_eq!(
        min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC".to_string()
    );
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
        min_window("bba".to_string(), "ab".to_string()),
        "ba".to_string()
    );
    assert_eq!(min_window("a".to_string(), "b".to_string()), "".to_string());
}

pub const NAME: &str = "min_window";
