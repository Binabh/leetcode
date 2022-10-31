pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left_pointer: usize = 0;
    let mut right_pointer = nums.len() - 1;
    let mut min = nums[left_pointer];
    while left_pointer <= right_pointer {
        if nums[left_pointer] <= nums[right_pointer] {
            min = i32::min(min, nums[left_pointer]);
            break;
        }
        let middle_pointer = (left_pointer + right_pointer) / 2;
        min = i32::min(min, nums[middle_pointer]);

        if nums[middle_pointer] >= nums[left_pointer] {
            left_pointer = middle_pointer + 1;
        } else {
            right_pointer = middle_pointer - 1;
        }
    }
    min
}
pub fn test() {
    assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(find_min(vec![4, 5, 6, 7]), 4);
    assert_eq!(find_min(vec![2, 1]), 1);
    assert_eq!(find_min(vec![1]), 1);
    assert_eq!(find_min(vec![3, 1, 2]), 1);
}

pub const NAME: &str = "find_min";
