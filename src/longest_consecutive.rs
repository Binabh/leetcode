use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let hash_set: HashSet<i32> = nums.into_iter().collect();

    for (_, num) in hash_set.iter().enumerate() {
        let left = num - 1;
        if !hash_set.contains(&left) {
            let mut length = 0;
            while hash_set.contains(&(num + length)) {
                length += 1;
            }
            if length > res {
                res = length;
            }
        }
    }
    return res;
}

pub fn test() {
    assert_eq!(longest_consecutive(vec![1, 2, 3, 4]), 4);
}

pub const NAME: &str = "longest_consecutive";
