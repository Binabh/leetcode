pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut stack: Vec<[i32; 2]> = Vec::new();
    for (i, h) in heights.iter().enumerate() {
        let mut start = i as i32;
        while stack.len() > 0 && stack.last().unwrap().last().unwrap() > h {
            let [index, height] = stack.pop().unwrap();
            let area = height * (i as i32 - index);
            if area > max_area {
                max_area = area;
            }
            start = index
        }
        stack.push([start, *h]);
    }
    for [i, height] in stack {
        let area = height * (heights.len() as i32 - i);
        if area > max_area {
            max_area = area;
        }
    }
    max_area
}

pub fn test() {
    assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(largest_rectangle_area(vec![2, 4]), 4)
}

pub const NAME: &str = "largest_rectangle_area";
