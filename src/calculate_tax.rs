#[cfg(test)]
mod tests {
    use std::cmp;
    fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut result = 0.0;
        let mut prev_brac: i32 = 0;
        for bracket in brackets {
            let taxed_amt = cmp::min(bracket[0] - prev_brac, income - prev_brac) as f64;
            result += taxed_amt * (bracket[1] as f64 / 100.0);
            if income - bracket[0] <= 0 {
                break;
            }
            prev_brac = bracket[0];
        }
        return result;
    }
    #[test]
    pub fn test() {
        assert_eq!(calculate_tax(vec![vec![2, 50]], 0), 0.0);
        assert_eq!(
            calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2),
            0.25000
        );
        assert_eq!(
            calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10),
            2.65000
        );
    }
}
