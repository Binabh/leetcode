#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let result: Vec<i32> = vec![];
    let (mut l, mut r) = (0 as usize, 0 as usize);

    result
}

pub fn test() {
    assert_eq!(
        max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
}

pub const NAME: &str = "max_sliding_window";
