pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums: Vec<i32> = Vec::new();
    nums.extend(nums1.iter());
    nums.extend(nums2.iter());
    nums.sort();
    let n = nums.len() as f32;
    let i = (&n / 2.0).floor() as usize;
    return if n == 0.0 {
        0.0
    } else if n % 2.0 == 1.0 {
        nums[i] as f64
    } else {
        ((nums[i - 1] as f64) + (nums[i] as f64)) / 2.0
    };
}

pub fn test() {
    assert_eq!(find_median_sorted_arrays(vec![], vec![]), 0.0)
}

pub const NAME: &str = "find_median_sorted_arrays";
