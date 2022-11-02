#[derive(Default)]
struct MinStack {
    data: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        return Default::default();
    }

    fn push(&mut self, val: i32) {
        if self.data.is_empty() {
            self.data.push((val, val));
        } else {
            let min_val = self.data.last().unwrap().1.min(val);
            self.data.push((val, min_val));
        }
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
pub fn test() {
    let mut obj = MinStack::new();
    obj.push(3);
    obj.push(4);
    assert_eq!(obj.top(), 4);
    assert_eq!(obj.get_min(), 3);
    obj.pop();
    assert_eq!(obj.top(), 3);
}

pub const NAME: &str = "min_stack";
