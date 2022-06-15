use std::collections::HashMap;

fn get_word_for_three(num: i32) -> String {
    let result = String::from("");

    let map1: HashMap<i32, String> = HashMap::from([
        (0, String::from("")),
        (1, String::from("One")),
        (2, String::from("Two")),
        (3, String::from("Three")),
        (4, String::from("Four")),
        (5, String::from("Five")),
        (6, String::from("Six")),
        (7, String::from("Seven")),
        (8, String::from("Eight")),
        (9, String::from("Nine")),
        (10, String::from("Ten")),
        (11, String::from("Eleven")),
        (12, String::from("Twelve")),
        (13, String::from("Thirteen")),
        (14, String::from("Fourteen")),
        (15, String::from("Fifteen")),
        (16, String::from("Sixteen")),
        (17, String::from("Seventeen")),
        (18, String::from("Eighteen")),
        (19, String::from("Nineteen")),
    ]);
    let map2: HashMap<i32, String> = HashMap::from([
        (2, String::from("Twenty")),
        (3, String::from("Thirty")),
        (4, String::from("Forty")),
        (5, String::from("Fifty")),
        (6, String::from("Sixty")),
        (7, String::from("Seventy")),
        (8, String::from("Eighty")),
        (9, String::from("Ninety")),
    ]);
    if num == 0 {
        result
    } else if num < 20 {
        map1.get(&num).unwrap().to_owned()
    } else if num < 100 {
        let new_num = num.to_owned() / 10;
        let mod_num = num.to_owned() % 10;
        map2.get(&new_num).unwrap().to_owned() + " " + &map1.get(&mod_num).unwrap().to_owned()
    } else {
        let new_num = num.to_owned() / 100;
        map1.get(&new_num).unwrap().to_owned() + " " + "Hundred " + &get_word_for_three(num % 100)
    }
}

pub fn number_to_words(mut num: i32) -> String {
    if num == 0 {
        return String::from("Zero");
    }
    let mut result = String::from("");
    let scales = ["", "Thousand", "Million", "Billion", "Trillion"];
    let mut i: usize = 0;
    while num > 0 {
        let sub_num = num % 1000;
        let three_digit_word =
            get_word_for_three(sub_num) + " " + if num % 1000 > 0 { scales[i] } else { "" } + " ";
        result.insert_str(0, &three_digit_word);
        i += 1;
        num /= 1000;
    }
    result
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .to_string()
}

pub fn test() {
    assert_eq!(number_to_words(0), String::from("Zero"));
    assert_eq!(number_to_words(50), String::from("Fifty"));
    assert_eq!(
        number_to_words(100000),
        String::from("One Hundred Thousand")
    );
    assert_eq!(number_to_words(1000000), String::from("One Million"));
    assert_eq!(
        number_to_words(100000000),
        String::from("One Hundred Million")
    );
    assert_eq!(number_to_words(10000000), String::from("Ten Million"));
    assert_eq!(number_to_words(1000000000), String::from("One Billion"));
    assert_eq!(number_to_words(1000000010), String::from("One Billion Ten"));
    assert_eq!(
        number_to_words(50868),
        String::from("Fifty Thousand Eight Hundred Sixty Eight")
    );
    assert_eq!(
        number_to_words(751868),
        String::from("Seven Hundred Fifty One Thousand Eight Hundred Sixty Eight")
    );
    assert_eq!(number_to_words(25), String::from("Twenty Five"));
}

pub const NAME: &str = "number_to_words";
