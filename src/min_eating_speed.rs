#[cfg(test)]
mod tests {
    fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = *piles.iter().max().unwrap();
        let mut res: i32 = right;
        while left <= right {
            let mid = (left + right) / 2;
            let mut hours: i64 = 0;
            for p in piles.iter() {
                hours = hours + ((p + (mid - 1)) / mid) as i64;
            }

            if hours <= h as i64 {
                res = res.min(mid);
                right = mid - 1;
            } else if hours > h as i64 {
                left = mid + 1;
            }
        }
        res
    }
    #[test]
    fn test() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
        assert_eq!(
            min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
            3
        );
    }
}
