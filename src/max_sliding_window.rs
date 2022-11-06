use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut q: VecDeque<usize> = VecDeque::new();
    let (mut l, mut r) = (0 as usize, 0 as usize);
    while r < nums.len() {
        while !q.is_empty() && nums[r] > nums[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(r);
        if l > *q.front().unwrap() {
            q.pop_front();
        }
        if r + 1 >= k as usize {
            result.push(nums[*q.front().unwrap()]);
            l += 1;
        }
        r += 1;
    }

    result
}

pub fn test() {
    assert_eq!(
        max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
}

pub const NAME: &str = "max_sliding_window";
