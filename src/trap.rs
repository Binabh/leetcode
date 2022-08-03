pub fn trap(height: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = height.len() - 1;
    let mut max_left: i32 = height[left_pointer];
    let mut max_right: i32 = height[right_pointer];
    while left_pointer < right_pointer {
        let water;
        if max_right > max_left {
            left_pointer += 1;
            water = max_left - height[left_pointer];
            if water > 0 {
                res += water;
            }
            if height[left_pointer] > max_left {
                max_left = height[left_pointer];
            }
        } else {
            right_pointer -= 1;
            water = max_right - height[right_pointer];
            if water > 0 {
                res += water;
            }
            if height[right_pointer] > max_right {
                max_right = height[right_pointer];
            }
        }
    }
    res
}

pub fn test() {
    assert_eq!(trap(vec![2, 2, 0, 2]), 2);
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9)
}

pub const NAME: &str = "trap";
