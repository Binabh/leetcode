pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut i = 0;
    while i < nums.len() {
        let mut j = i + 1;
        while j < nums.len() {
            let mut k = j + 1;
            while k < nums.len() {
                if nums[i] != nums[j] && nums[i] != nums[k] && nums[k] != nums[j] {
                    result += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    result
}

pub fn test() {
    assert_eq!(unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
    assert_eq!(unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(unequal_triplets(vec![1, 2, 1]), 0);
    assert_eq!(unequal_triplets(vec![1, 3, 1, 2, 4]), 7);
}

pub const NAME: &str = "unequal_triplets";
