#[cfg(test)]
mod tests {
    fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut stack: Vec<f64> = Vec::new();
        let mut merged_vec: Vec<[i32; 2]> = Vec::new();
        for (i, p) in position.iter().enumerate() {
            merged_vec.push([*p, speed[i]]);
        }
        merged_vec.sort_by(|a, b| b.first().unwrap().partial_cmp(a.first().unwrap()).unwrap());
        for [pos, sp] in merged_vec {
            stack.push((target - pos) as f64 / sp as f64);
            if stack.len() >= 2 && *stack.last().unwrap() <= stack[stack.len() - 2] {
                stack.pop();
            }
        }
        stack.len() as i32
    }
    #[test]
    fn test() {
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
        assert_eq!(car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
        assert_eq!(car_fleet(10, vec![3], vec![3]), 1);
    }
}
