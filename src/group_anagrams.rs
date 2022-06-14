use std::collections::{hash_map::Entry, HashMap};

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    if strs == vec![String::from("")] {
        return vec![strs];
    }
    let mut result = vec![];
    let mut anagram_map: HashMap<[usize; 27], Vec<String>> = HashMap::new();
    for word in strs {
        let mut key_array: [usize; 27] = [0; 27];
        for ch in word.chars() {
            let index = ch as u32 - 'a' as u32;
            let uindex = index as usize;
            key_array[uindex] = key_array[uindex] + 1;
        }
        match anagram_map.entry(key_array) {
            Entry::Vacant(e) => {
                e.insert(vec![word]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(word);
            }
        }
    }
    for (_, value) in anagram_map {
        result.push(value);
    }
    result
}

pub fn test() {
    assert_eq!(
        group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ]),
        vec![
            vec![String::from("bat")],
            vec![
                String::from("eat"),
                String::from("tea"),
                String::from("ate")
            ],
            vec![String::from("tan"), String::from("nat")]
        ]
    )
}

pub const NAME: &str = "group_anagrams";
