use std::ffi::OsStr;
use std::path::Path;

use crate::quizzes::week1::{
    breaking_the_records,
    camel_case_4,
    divisible_sum_pairs,
    mini_max_sum,
    plus_minus,
    sparse_arrays,
    time_conversion,
};
use crate::quizzes::week2::{
    counting_sort_1,
    counting_valleys,
    diagonal_difference,
    flipping_bits,
    grading_students,
    lonely_integer,
    mars_exploration,
    pangrams,
};
use crate::quizzes::week3::{
    drawing_book,
    maximum_perimeter_triangle,
    migratory_birds,
    permuting_two_arrays,
    sales_by_match,
    subarray_division_2,
    xor_strings_3,
    zig_zag_sequence,
};

use super::types::{Quiz, QuizOption};
// use crate::quizzes::week4::{
//     picking_numbers,
//     left_rotation,
//     number_line_jumps,
//     separate_the_numbers,
//     closest_numbers,
//     tower_breakers,
//     minimum_absolute_difference_in_an_array,
//     caesar_cipher,
// };
// use crate::quizzes::week5::{
//     max_min,
//     strong_password,
//     dynamic_array,
//     smart_number_2,
//     missing_numbers,
//     the_full_counting_sort,
//     grid_challenge,
//     sansa_and_xor,
// };
// use crate::quizzes::week6::{
//     prime_dates,
//     sherlock_and_array,
//     misère_nim,
//     gaming_array_1,
//     forming_a_magic_square,
//     recursive_digit_sum,
//     counter_game,
//     sum_vs_xor,
// };
// use crate::quizzes::week7::{
//     climbing_the_leaderboard,
//     the_bomberman_game,
//     new_year_chaos,
//     goodland_electricity,
//     sherlock_and_the_valid_string,
// };

pub fn test(quiz: Quiz) {
    match quiz.name {
        // WEEK 1
        QuizOption::PlusMinus => _ = plus_minus::test(&quiz.input),
        QuizOption::MiniMaxSum => _ = mini_max_sum::test(&quiz.input),
        QuizOption::TimeConversion => _ = time_conversion::test(&quiz.input),
        QuizOption::BreakingTheRecords => _ = breaking_the_records::test(&quiz.input),
        QuizOption::CamelCase4 => _ = camel_case_4::test(&quiz.input),
        QuizOption::DivisibleSumPairs => _ = divisible_sum_pairs::test(&quiz.input),
        QuizOption::SparseArrays => _ = sparse_arrays::test(&quiz.input),
        // WEEK 2
        QuizOption::LonelyInteger => _ = lonely_integer::test(&quiz.input),
        QuizOption::GradingStudents => _ = grading_students::test(&quiz.input),
        QuizOption::FlippingBits => _ = flipping_bits::test(&quiz.input),
        QuizOption::DiagonalDifference => _ = diagonal_difference::test(&quiz.input),
        QuizOption::CountingSort1 => _ = counting_sort_1::test(&quiz.input),
        QuizOption::CountingValleys => _ = counting_valleys::test(&quiz.input),
        QuizOption::Pangrams => _ = pangrams::test(&quiz.input),
        QuizOption::MarsExploration => _ = mars_exploration::test(&quiz.input),
        // WEEK 3
        QuizOption::PermutingTwoArrays => _ = permuting_two_arrays::test(&quiz.input),
        QuizOption::SubarrayDivision2 => _ = subarray_division_2::test(&quiz.input),
        QuizOption::XorStrings3 => _ = xor_strings_3::test(&quiz.input),
        QuizOption::SalesByMatch => _ = sales_by_match::test(&quiz.input),
        QuizOption::MigratoryBirds => _ = migratory_birds::test(&quiz.input),
        QuizOption::MaximumPerimeterTriangle => _ = maximum_perimeter_triangle::test(&quiz.input),
        QuizOption::ZigZagSequence => _ = zig_zag_sequence::test(&quiz.input),
        QuizOption::DrawingBook => _ = drawing_book::test(&quiz.input),
    }
}

pub fn filename(file: &str) -> &OsStr {
    let path_from_dotenv = Path::new(file);
    let filename = path_from_dotenv.file_name().expect("works");
    filename
}