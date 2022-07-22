use std::collections::{hash_map::Entry, HashMap};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match hash.entry(*num) {
            Entry::Occupied(_) => {
                return true;
            }
            Entry::Vacant(v) => {
                v.insert(i);
            }
        }
    }
    return false;
}

pub fn test() {
    assert_eq!(contains_duplicate(vec![1, 2, 3]), false);
    assert_eq!(contains_duplicate(vec![1, 2, 2]), true);
}

pub const NAME: &str = "contains_duplicate";
