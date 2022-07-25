use std::collections::HashSet;

fn two_sum_sorted(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;
    let mut res = vec![];
    while left < right {
        if (nums[left] + nums[right]) < target {
            left += 1;
        } else if (nums[left] + nums[right]) > target {
            right -= 1;
        } else if (nums[left] + nums[right]) == target {
            res.push(vec![nums[left], nums[right]]);
            left += 1;
            right -= 1;
        }
    }
    res
}
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: HashSet<Vec<i32>> = HashSet::new();
    nums.sort();
    for (i, n) in nums[..nums.len() - 2].iter().enumerate() {
        let res_two_sum = two_sum_sorted((&nums[(i + 1)..]).to_vec(), 0 - n);
        for (_, e) in res_two_sum.iter().enumerate() {
            let mut f = vec![*n];
            f.extend(e);
            res.insert(f);
        }
    }
    res.into_iter().collect()
}

pub fn test() {
    assert_eq!(three_sum(vec![1, 2, -3]), vec![[-3, 1, 2]]);
    assert_eq!(three_sum(vec![0, 0, 0]), vec![[0, 0, 0]]);
    assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![[0, 0, 0]]);
    assert_eq!(three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
    assert_eq!(
        three_sum(vec![-2, 0, 1, 1, 2]),
        vec![[-2, 0, 2], [-2, 1, 1]]
    );
    assert_eq!(
        three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![[-1, -1, 2], [-1, 0, 1]]
    );
}

pub const NAME: &str = "three_sum";
