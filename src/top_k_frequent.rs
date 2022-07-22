use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count_vec: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
    let mut hash: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = vec![];
    for (_, num) in nums.iter().enumerate() {
        match hash.get_mut(num) {
            Some(v) => *v += 1,
            None => {
                hash.insert(*num, 1);
            }
        }
    }
    for (num, count) in hash.iter() {
        let element = &mut count_vec[*count];
        element.push(*num);
    }
    for (_, vecs) in count_vec.iter().rev().enumerate() {
        result.extend(vecs);
        if result.len() as i32 >= k {
            break;
        }
    }
    return result;
}

pub fn test() {
    assert_eq!(top_k_frequent(vec![1, 2, 2], 1), vec![2]);
    assert_eq!(top_k_frequent(vec![1, 2, 2, 3, 3], 2), vec![3, 2]);
    assert_eq!(top_k_frequent(vec![1, 2, 2], 2), vec![2, 1]);
}

pub const NAME: &str = "top_k_frequent";
