#[cfg(test)]
mod tests {
    fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_vec: [usize; 27] = [0; 27];
        for ch in s.chars() {
            let index = ch as u32 - 'a' as u32;
            let uindex = index as usize;
            s_vec[uindex] = s_vec[uindex] + 1;
        }
        let mut t_vec: [usize; 27] = [0; 27];
        for ch in t.chars() {
            let index = ch as u32 - 'a' as u32;
            let uindex = index as usize;
            t_vec[uindex] = t_vec[uindex] + 1;
        }
        t_vec == s_vec
    }
    #[test]
    fn test() {
        assert_eq!(
            is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );
        assert_eq!(is_anagram(String::from("rat"), String::from("car")), false);
    }
}
