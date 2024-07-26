#[cfg(test)]
mod tests {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left_pointer: usize = 0;
        let mut right_pointer = nums.len() - 1;
        while left_pointer <= right_pointer {
            let middle_pointer = (left_pointer + right_pointer) / 2;
            if target == nums[middle_pointer] {
                return middle_pointer as i32;
            }
            if nums[left_pointer] <= nums[middle_pointer] {
                if target > nums[middle_pointer] || target < nums[left_pointer] {
                    left_pointer = middle_pointer + 1;
                } else {
                    right_pointer = middle_pointer - 1;
                }
            } else {
                if target < nums[middle_pointer] || target > nums[right_pointer] {
                    right_pointer = middle_pointer - 1;
                } else {
                    left_pointer = middle_pointer + 1;
                }
            }
        }
        -1
    }
    #[test]
    fn test() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(search(vec![1], 1), 0);
        assert_eq!(search(vec![1], 3), -1);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2, 3], 3), 7);
    }
}
