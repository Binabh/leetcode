mod apply_operations;
mod calculate_tax;
mod car_fleet;
mod character_replacement;
mod check_inclusion;
mod contains_duplicate;
mod convert_temperature;
mod convert_to_base7;
mod daily_temperatures;
mod distinct_averages;
mod divide;
mod eval_rpn;
mod find_median_sorted_arrays;
mod find_min;
mod generate_parenthesis;
mod group_anagrams;
mod int_to_roman;
mod is_anagram;
mod is_palindrome;
mod is_valid;
mod is_valid_sudoku;
mod largest_rectangle_area;
mod length_of_longest_substring;
mod letter_combinations;
mod longest_common_prefix;
mod longest_consecutive;
mod max_area;
mod max_palindromes;
mod max_profit;
mod max_sliding_window;
mod min_eating_speed;
mod min_stack;
mod min_window;
mod my_pow;
mod my_sqrt;
mod number_to_words;
mod product_except_self;
mod reverse;
mod reverse_list;
mod roman_to_int;
mod search;
mod search_martix;
mod subarray_lcm;
mod three_sum;
mod time_map;
mod top_k_frequent;
mod trap;
mod two_sum;
mod unequal_triplets;
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
        letter_combinations::NAME => letter_combinations::test(),
        contains_duplicate::NAME => contains_duplicate::test(),
        top_k_frequent::NAME => top_k_frequent::test(),
        daily_temperatures::NAME => daily_temperatures::test(),
        search_martix::NAME => search_martix::test(),
        is_valid_sudoku::NAME => is_valid_sudoku::test(),
        longest_consecutive::NAME => longest_consecutive::test(),
        three_sum::NAME => three_sum::test(),
        max_area::NAME => max_area::test(),
        trap::NAME => trap::test(),
        max_profit::NAME => max_profit::test(),
        length_of_longest_substring::NAME => length_of_longest_substring::test(),
        character_replacement::NAME => character_replacement::test(),
        check_inclusion::NAME => check_inclusion::test(),
        min_window::NAME => min_window::test(),
        search::NAME => search::test(),
        find_min::NAME => find_min::test(),
        max_sliding_window::NAME => max_sliding_window::test(),
        eval_rpn::NAME => eval_rpn::test(),
        min_stack::NAME => min_stack::test(),
        apply_operations::NAME => apply_operations::test(),
        distinct_averages::NAME => distinct_averages::test(),
        convert_temperature::NAME => convert_temperature::test(),
        subarray_lcm::NAME => subarray_lcm::test(),
        max_palindromes::NAME => max_palindromes::test(),
        unequal_triplets::NAME => unequal_triplets::test(),
        generate_parenthesis::NAME => generate_parenthesis::test(),
        car_fleet::NAME => car_fleet::test(),
        largest_rectangle_area::NAME => largest_rectangle_area::test(),
        min_eating_speed::NAME => min_eating_speed::test(),
        product_except_self::NAME => product_except_self::test(),
        time_map::NAME => time_map::test(),
        reverse_list::NAME => reverse_list::test(),
        _ => print!("{}", query),
    }
}
