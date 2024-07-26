#[cfg(test)]
mod tests {
    fn int_to_roman(mut num: i32) -> String {
        let mut result = String::from("");

        let ones = ['I', 'X', 'C', 'M'];
        let fives = ['V', 'L', 'D'];
        let mut i: usize = 0;
        while num != 0 {
            let digit = num % 10;
            let numeral: Vec<char> = match digit {
                1 => vec![ones[i]],
                2 => vec![ones[i]; 2],
                3 => vec![ones[i]; 3],
                4 => vec![ones[i], fives[i]],
                5 => vec![fives[i]],
                6 => vec![fives[i], ones[i]],
                7 => vec![fives[i], ones[i], ones[i]],
                8 => vec![fives[i], ones[i], ones[i], ones[i]],
                9 => vec![ones[i], ones[i + 1]],
                _ => Vec::new(),
            };
            let final_numeral: String = numeral.into_iter().collect();
            result.insert_str(0, &final_numeral);
            num /= 10;
            i += 1;
        }
        result
    }
    #[test]
    fn test() {
        assert_eq!(int_to_roman(3), String::from("III"))
    }
}
