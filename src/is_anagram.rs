use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut map: HashMap<char, usize> = HashMap::new();
    for ch in s.chars() {
        match map.get(&ch) {
            Some(_) => {
                continue;
            }
            None => {
                let scount = s.matches(ch).count();
                let tcount = t.matches(ch).count();
                if scount != tcount {
                    return false;
                }
                map.insert(ch, scount);
            }
        };
    }
    true
}
pub fn test() {
    assert_eq!(
        is_anagram(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
}

pub const NAME: &str = "is_anagram";
