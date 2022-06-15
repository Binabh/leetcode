mod calculate_tax;
mod convert_to_base7;
mod divide;
mod find_median_sorted_arrays;
mod group_anagrams;
mod int_to_roman;
mod is_anagram;
mod is_palindrome;
mod is_valid;
mod longest_common_prefix;
mod my_pow;
mod my_sqrt;
mod number_to_words;
mod reverse;
mod roman_to_int;
mod two_sum;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args[1].as_str();
    match query {
        two_sum::NAME => two_sum::test(),
        is_palindrome::NAME => is_palindrome::test(),
        is_valid::NAME => is_valid::test(),
        my_sqrt::NAME => my_sqrt::test(),
        my_pow::NAME => my_pow::test(),
        reverse::NAME => reverse::test(),
        roman_to_int::NAME => roman_to_int::test(),
        longest_common_prefix::NAME => longest_common_prefix::test(),
        find_median_sorted_arrays::NAME => find_median_sorted_arrays::test(),
        divide::NAME => divide::test(),
        calculate_tax::NAME => calculate_tax::test(),
        is_anagram::NAME => is_anagram::test(),
        group_anagrams::NAME => group_anagrams::test(),
        int_to_roman::NAME => int_to_roman::test(),
        number_to_words::NAME => number_to_words::test(),
        convert_to_base7::NAME => convert_to_base7::test(),
        _ => print!("{}", query),
    }
}
