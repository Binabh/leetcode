use std::collections::HashMap;

struct TimeUnit {
    timestamp: i32,
    value: String,
}

#[derive(Default)]
struct TimeMap {
    store: HashMap<String, Vec<TimeUnit>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        return Default::default();
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store
            .entry(key)
            .or_insert(Vec::new())
            .push(TimeUnit { timestamp, value });
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let mut res = "".to_string();
        let values = match self.store.get(&key) {
            Some(d) => d,
            None => {
                return res;
            }
        };
        let mut left: usize = 0;
        let mut right = values.len() - 1;
        while left <= right {
            let m: usize = (left + right) / 2;
            if values[m].timestamp <= timestamp {
                res = values[m].value.clone();
                left = m + 1;
            } else {
                if m < 1 {
                    break;
                }
                right = m - 1;
            }
        }
        res
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
pub fn test() {
    let mut obj = TimeMap::new();
    obj.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(obj.get("foo".to_string(), 1), "bar".to_string());
    assert_eq!(obj.get("foo".to_string(), 3), "bar".to_string());
    obj.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(obj.get("foo".to_string(), 4), "bar2".to_string());
    assert_eq!(obj.get("foo".to_string(), 5), "bar2".to_string());
    obj.set("love".to_string(), "high".to_string(), 10);
    obj.set("love".to_string(), "low".to_string(), 20);
    assert_eq!(obj.get("love".to_string(), 5), "".to_string());
    assert_eq!(obj.get("love".to_string(), 15), "high".to_string());
    assert_eq!(obj.get("love".to_string(), 20), "low".to_string());
    assert_eq!(obj.get("love".to_string(), 25), "low".to_string());
    let mut obj = TimeMap::new();
    obj.set("a".to_string(), "bar".to_string(), 1);
    obj.set("x".to_string(), "b".to_string(), 3);
    assert_eq!(obj.get("b".to_string(), 3), "".to_string());
    obj.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(obj.get("foo".to_string(), 4), "bar2".to_string());
    assert_eq!(obj.get("foo".to_string(), 5), "bar2".to_string());
}

pub const NAME: &str = "time_map";
