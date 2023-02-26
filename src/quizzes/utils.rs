use std::ffi::OsStr;
use std::path::Path;

use crate::quizzes::level1::{
    breaking_the_records, camel_case_4, divisible_sum_pairs, mini_max_sum, plus_minus,
    sparse_arrays, time_conversion,
};
use crate::quizzes::level2::{
    counting_sort_1, counting_valleys, diagonal_difference, flipping_bits, grading_students,
    lonely_integer, mars_exploration, pangrams,
};
use crate::quizzes::level3::{
    drawing_book, maximum_perimeter_triangle, migratory_birds, permuting_two_arrays,
    sales_by_match, subarray_division_2, xor_strings_3, zig_zag_sequence,
};
use crate::quizzes::types::QuizConfig;

use super::types::{Quiz};
// use crate::quizzes::level4::{
//     picking_numbers,
//     left_rotation,
//     number_line_jumps,
//     separate_the_numbers,
//     closest_numbers,
//     tower_breakers,
//     minimum_absolute_difference_in_an_array,
//     caesar_cipher,
// };
// use crate::quizzes::level5::{
//     max_min,
//     strong_password,
//     dynamic_array,
//     smart_number_2,
//     missing_numbers,
//     the_full_counting_sort,
//     grid_challenge,
//     sansa_and_xor,
// };
// use crate::quizzes::level6::{
//     prime_dates,
//     sherlock_and_array,
//     misère_nim,
//     gaming_array_1,
//     forming_a_magic_square,
//     recursive_digit_sum,
//     counter_game,
//     sum_vs_xor,
// };
// use crate::quizzes::level7::{
//     climbing_the_leaderboard,
//     the_bomberman_game,
//     new_year_chaos,
//     goodland_electricity,
//     sherlock_and_the_valid_string,
// };

pub fn quiz(quiz: Quiz) {
    match &quiz.name as &str {
        // WEEK 1
        "plus_minus" => _ = plus_minus::quiz(),
        "mini_max_sum" => _ = mini_max_sum::quiz(),
        "time_conversion" => _ = time_conversion::quiz(),
        "breaking_the_records" => _ = breaking_the_records::quiz(),
        "camel_case_4" => _ = camel_case_4::quiz(),
        "divisible_sum_pairs" => _ = divisible_sum_pairs::quiz(),
        "sparse_arrays" => _ = sparse_arrays::quiz(),
        // WEEK 2
        "lonely_integer" => _ = lonely_integer::quiz(),
        "grading_students" => _ = grading_students::quiz(),
        "flipping_bits" => _ = flipping_bits::quiz(),
        "diagonal_difference" => _ = diagonal_difference::quiz(),
        "counting_sort_1" => _ = counting_sort_1::quiz(),
        "counting_valleys" => _ = counting_valleys::quiz(),
        "pangrams" => _ = pangrams::quiz(),
        "mars_exploration" => _ = mars_exploration::quiz(),
        // WEEK 3
        "permuting_two_arrays" => _ = permuting_two_arrays::quiz(),
        "subarray_division_2" => _ = subarray_division_2::quiz(),
        "xor_strings_3" => _ = xor_strings_3::quiz(),
        "sales_by_match" => _ = sales_by_match::quiz(),
        "migratory_birds" => _ = migratory_birds::quiz(),
        "maximum_perimeter_triangle" => _ = maximum_perimeter_triangle::quiz(),
        "zig_zag_sequence" => _ = zig_zag_sequence::quiz(),
        "drawing_book" => _ = drawing_book::quiz(),
        &_ => {},
    }
}


pub fn get_config(quiz: Quiz) -> QuizConfig {
    let config:QuizConfig;
    match &quiz.name as &str {
        // WEEK 1
        "plus_minus" => config = plus_minus::config(),
        "mini_max_sum" => config = mini_max_sum::config(),
        "time_conversion" => config = time_conversion::config(),
        "breaking_the_records" => config = breaking_the_records::config(),
        "camel_case_4" => config = camel_case_4::config(),
        "divisible_sum_pairs" => config = divisible_sum_pairs::config(),
        "sparse_arrays" => config = sparse_arrays::config(),
        // WEEK 2
        "lonely_integer" => config = lonely_integer::config(),
        "grading_students" => config = grading_students::config(),
        "flipping_bits" => config = flipping_bits::config(),
        "diagonal_difference" => config = diagonal_difference::config(),
        "counting_sort_1" => config = counting_sort_1::config(),
        "counting_valleys" => config = counting_valleys::config(),
        "pangrams" => config = pangrams::config(),
        "mars_exploration" => config = mars_exploration::config(),
        // WEEK 3
        "permuting_two_arrays" => config = permuting_two_arrays::config(),
        "subarray_division_2" => config = subarray_division_2::config(),
        "xor_strings_3" => config = xor_strings_3::config(),
        "sales_by_match" => config = sales_by_match::config(),
        "migratory_birds" => config = migratory_birds::config(),
        "maximum_perimeter_triangle" => config = maximum_perimeter_triangle::config(),
        "zig_zag_sequence" => config = zig_zag_sequence::config(),
        "drawing_book" => config = drawing_book::config(),
        &_ => config = plus_minus::config(),
    }
    config
}

pub fn filename(file: &str) -> &OsStr {
    let path_from_dotenv = Path::new(file);
    let filename = path_from_dotenv.file_name().expect("works");
    filename
}

use std::fs;
pub fn read_from_input_file(level: &str, name: &str) -> String {
    // load file or panic
    let path = format!("./src/quizzes/{}/{}.txt", level, name);
    let file = fs::read_to_string(path).unwrap();
    file
}