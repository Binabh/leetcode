pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; temperatures.len()];
    let mut stack: Vec<[i32; 2]> = Vec::new();
    for (idx, temp) in temperatures.iter().enumerate() {
        while *temp
            > match stack.last() {
                Some([v, _]) => *v,
                None => i32::MAX,
            }
        {
            let [_, i] = stack.pop().unwrap();
            result[i as usize] = idx as i32 - i;
        }
        stack.push([*temp, idx as i32]);
    }
    result
}

pub fn test() {
    assert_eq!(
        daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
}

pub const NAME: &str = "daily_temperatures";
