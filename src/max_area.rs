pub fn max_area(height: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;
    while left < right {
        let rh = height[right];
        let lh = height[left];
        let area;
        if rh > lh {
            area = lh * (right - left) as i32;
            left += 1;
        } else {
            area = rh * (right - left) as i32;
            right -= 1;
        }
        if area > res {
            res = area;
        }
    }
    res
}

pub fn test() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
}

pub const NAME: &str = "max_area";
