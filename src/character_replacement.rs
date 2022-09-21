pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = left_pointer;
    let mut count: [usize; 26] = [0; 26];
    let s_chars: Vec<char> = s.chars().collect();
    let mut result = right_pointer - left_pointer + 1;
    while right_pointer < s.len() {
        let rt_count = &mut count[s_chars[right_pointer] as usize - 'A' as usize];
        *rt_count += 1;
        let maximum = *count.iter().max().unwrap();
        while right_pointer - left_pointer + 1 - maximum > k as usize {
            count[s_chars[left_pointer] as usize - 'A' as usize] -= 1;
            left_pointer += 1;
        }
        result = std::cmp::max(result, right_pointer - left_pointer + 1);
        right_pointer += 1;
    }
    result as i32
}

pub fn test() {
    assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
    assert_eq!(character_replacement("ABBB".to_string(), 2), 4);
    assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    assert_eq!(character_replacement("KRSCDCSONAJNHLBMDQGIFCPEKPOHQIHLTDIQGEKLRLCQNBOHNDQGHJPNDQPERNFSSSRDEQLFPCCCARFMDLHADJADAGNNSBNCJQOF".to_string(), 4), 7);
}

pub const NAME: &str = "character_replacement";
