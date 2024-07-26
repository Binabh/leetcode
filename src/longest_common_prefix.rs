#[cfg(test)]
mod tests {
    fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let mut lcp = String::from("");
        strs.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut triangle: Vec<String> = vec![String::from("")];
        let shortest_string: String = match strs.get(0) {
            Some(s) => s.to_owned(),
            None => String::from(""),
        };
        for ch in shortest_string.chars() {
            let mut last_substring: String = match triangle.last() {
                Some(s) => s.to_owned(),
                None => String::from(""),
            };
            last_substring.push(ch);
            triangle.push(last_substring);
        }
        for st in strs {
            let new_triangle = triangle.to_owned();
            for sub in new_triangle.iter().rev() {
                if st.starts_with(sub) || st.ends_with(sub) {
                    lcp = sub.to_owned();
                    break;
                } else {
                    triangle.retain(|x| !x.eq(sub));
                }
            }
        }
        lcp
    }
    #[test]
    fn test() {
        assert_eq!(longest_common_prefix(vec![]), "")
    }
}
