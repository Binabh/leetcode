#[cfg(test)]
mod tests {
    fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        result
    }
    #[test]
    fn test() {
        assert_eq!(subarray_lcm(vec![3, 6, 2, 7, 1], 6), 4);
        assert_eq!(subarray_lcm(vec![3], 2), 0);
    }
}
