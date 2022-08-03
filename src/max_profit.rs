pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res: i32 = 0;

    if prices.len() < 2 {
        return res;
    }
    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = left_pointer + 1;
    let mut min_left: i32 = prices[left_pointer];
    while right_pointer <= (prices.len() - 1) {
        if prices[right_pointer] - min_left > res {
            res = prices[right_pointer] - min_left;
        }
        if min_left > prices[right_pointer] {
            left_pointer = right_pointer;
            min_left = prices[left_pointer];
        }
        right_pointer += 1;
    }

    res
}

pub fn test() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(max_profit(vec![1, 2]), 1);
    assert_eq!(max_profit(vec![2, 1, 2, 1, 0, 1, 2]), 2);
    assert_eq!(max_profit(vec![9, 7, 1, 6, 0, 3]), 5);
}

pub const NAME: &str = "max_profit";
