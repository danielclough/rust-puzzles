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
        "plus_minus" => _ = plus_minus::quiz(&quiz.input),
        "mini_max_sum" => _ = mini_max_sum::quiz(&quiz.input),
        "time_conversion" => _ = time_conversion::quiz(&quiz.input),
        "breaking_the_records" => _ = breaking_the_records::quiz(&quiz.input),
        "camel_case_4" => _ = camel_case_4::quiz(&quiz.input),
        "divisible_sum_pairs" => _ = divisible_sum_pairs::quiz(&quiz.input),
        "sparse_arrays" => _ = sparse_arrays::quiz(&quiz.input),
        // WEEK 2
        "lonely_integer" => _ = lonely_integer::quiz(&quiz.input),
        "grading_students" => _ = grading_students::quiz(&quiz.input),
        "flipping_bits" => _ = flipping_bits::quiz(&quiz.input),
        "diagonal_difference" => _ = diagonal_difference::quiz(&quiz.input),
        "counting_sort_1" => _ = counting_sort_1::quiz(&quiz.input),
        "counting_valleys" => _ = counting_valleys::quiz(&quiz.input),
        "pangrams" => _ = pangrams::quiz(&quiz.input),
        "mars_exploration" => _ = mars_exploration::quiz(&quiz.input),
        // WEEK 3
        "permuting_two_arrays" => _ = permuting_two_arrays::quiz(&quiz.input),
        "subarray_division_2" => _ = subarray_division_2::quiz(&quiz.input),
        "xor_strings_3" => _ = xor_strings_3::quiz(&quiz.input),
        "sales_by_match" => _ = sales_by_match::quiz(&quiz.input),
        "migratory_birds" => _ = migratory_birds::quiz(&quiz.input),
        "maximum_perimeter_triangle" => _ = maximum_perimeter_triangle::quiz(&quiz.input),
        "zig_zag_sequence" => _ = zig_zag_sequence::quiz(&quiz.input),
        "drawing_book" => _ = drawing_book::quiz(&quiz.input),
        &_ => {},
    }
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