pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i64> = vec![1; nums.len()];
    let mut cum_prod: i64 = 1;

    for (index, num) in nums.iter().enumerate() {
        if *num == 0 {
            result[index] = cum_prod;
            cum_prod *= *num as i64;
        } else {
            cum_prod *= *num as i64;
            result[index] = cum_prod;
        }
    }
    cum_prod = 1;
    for (index, _) in nums.iter().enumerate() {
        let actual_index = nums.len() - 1 - index;
        if nums[actual_index] == 0 {
            result[actual_index] = cum_prod * result[actual_index];
            cum_prod *= nums[actual_index] as i64;
        } else {
            cum_prod *= nums[actual_index] as i64;
            result[actual_index] =
                result[actual_index] * cum_prod / ((nums[actual_index] * nums[actual_index]) as i64)
        }
    }
    return result.iter().map(|n| *n as i32).collect();
}

pub fn test() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(
        product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
    assert_eq!(product_except_self(vec![1, 0]), vec![0, 1]);
    assert_eq!(
        product_except_self(vec![5, 9, 2, -9, -9, -7, -8, 7, -9, 10]),
        vec![
            -51438240, -28576800, -128595600, 28576800, 28576800, 36741600, 32148900, -36741600,
            28576800, -25719120
        ]
    );
}

pub const NAME: &str = "product_except_self";
