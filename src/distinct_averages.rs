use std::collections::HashSet;

pub fn distinct_averages(nums: Vec<i32>) -> i32 {
    let mut res: HashSet<(i32, usize)> = HashSet::new();
    let mut new_nums = nums.to_vec();
    new_nums.sort();
    let mut l: usize = 0;
    let mut r: usize = new_nums.len() - 1;
    while l < r {
        if (new_nums[l] + new_nums[r]) % 2 == 0 {
            res.insert(((new_nums[l] + new_nums[r]) / 2, 0));
        } else {
            res.insert(((new_nums[l] + new_nums[r]) / 2, 1));
        }
        l += 1;
        r -= 1;
    }
    res.len() as i32
}

pub fn test() {
    assert_eq!(distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
    assert_eq!(distinct_averages(vec![1, 100]), 1);
    assert_eq!(distinct_averages(vec![10, 2, 2, 0, 4, 0]), 2);
}

pub const NAME: &str = "distinct_averages";
