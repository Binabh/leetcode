use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(&complement_index) = hash.get(num) {
            return vec![complement_index as i32, i as i32];
        }
        hash.insert((target - num) as i32, i);
    }
    vec![]
}
pub fn two_sum_sorted(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;
    loop {
        if (nums[left] + nums[right]) < target {
            left += 1;
        } else if (nums[left] + nums[right]) > target {
            right -= 1;
        } else if (nums[left] + nums[right]) == target {
            return vec![(left + 1) as i32, (right + 1) as i32];
        }
    }
}

pub fn test() {
    assert_eq!(two_sum(vec![1, 2, 3], 5), vec![1, 2]);
    assert_eq!(two_sum_sorted(vec![1, 2, 3], 5), vec![2, 3]);
    assert_eq!(two_sum_sorted(vec![1, 2, 3, 4, 7, 20], 24), vec![4, 6]);
}

pub const NAME: &str = "two_sum";
