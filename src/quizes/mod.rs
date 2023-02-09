use crate::Quiz;
pub mod week1;
use crate::quizes::week1::plus_minus;
use crate::quizes::week1::mini_max_sum;
use crate::quizes::week1::time_conversion;
use crate::quizes::week1::breaking_the_records;
use crate::quizes::week1::camel_case_4;
use crate::quizes::week1::divisible_sum_pairs;
use crate::quizes::week1::sparse_arrays;

pub mod week2;
use crate::quizes::week2::lonely_integer;
use crate::quizes::week2::grading_students;
use crate::quizes::week2::flipping_bits;
use crate::quizes::week2::diagonal_difference;
use crate::quizes::week2::counting_sort_1;
use crate::quizes::week2::counting_valleys;
use crate::quizes::week2::pangrams;
use crate::quizes::week2::mars_exploration;

pub mod week3;
use crate::quizes::week3::permuting_two_arrays;
use crate::quizes::week3::subarray_division_2;
use crate::quizes::week3::xor_strings_3;
use crate::quizes::week3::sales_by_match;
use crate::quizes::week3::migratory_birds;
use crate::quizes::week3::maximum_perimeter_triangle;
use crate::quizes::week3::zig_zag_sequence;
use crate::quizes::week3::drawing_book;

// pub mod week4;
// use crate::quizes::week4::picking_numbers;
// use crate::quizes::week4::left_rotation;
// use crate::quizes::week4::number_line_jumps;
// use crate::quizes::week4::separate_the_numbers;
// use crate::quizes::week4::closest_numbers;
// use crate::quizes::week4::tower_breakers;
// use crate::quizes::week4::minimum_absolute_difference_in_an_array;
// use crate::quizes::week4::caesar_cipher;

// pub mod week5;
// use crate::quizes::week5::max_min;
// use crate::quizes::week5::strong_password;
// use crate::quizes::week5::dynamic_array;
// use crate::quizes::week5::smart_number_2;
// use crate::quizes::week5::missing_numbers;
// use crate::quizes::week5::the_full_counting_sort;
// use crate::quizes::week5::grid_challenge;
// use crate::quizes::week5::sansa_and_xor;

// pub mod week6;
// use crate::quizes::week6::prime_dates;
// use crate::quizes::week6::sherlock_and_array;
// use crate::quizes::week6::misère_nim;
// use crate::quizes::week6::gaming_array_1;
// use crate::quizes::week6::forming_a_magic_square;
// use crate::quizes::week6::recursive_digit_sum;
// use crate::quizes::week6::counter_game;
// use crate::quizes::week6::sum_vs_xor;

// pub mod week7;
// use crate::quizes::week7::climbing_the_leaderboard;
// use crate::quizes::week7::the_bomberman_game;
// use crate::quizes::week7::new_year_chaos;
// use crate::quizes::week7::goodland_electricity;
// use crate::quizes::week7::sherlock_and_the_valid_string;

pub fn match_quiz(quiz: Quiz) {
    println!("Quiz:\n\t{}\n", &quiz.name);

    match quiz.name.as_str() {
        // WEEK 1
        "plus_minus" => _ = plus_minus::test(&quiz.input),
        "mini_max_sum" => _ = mini_max_sum::test(&quiz.input),
        "time_conversion" => _ = time_conversion::test(&quiz.input),
        "breaking_the_records" => _ = breaking_the_records::test(&quiz.input),
        "camel_case_4" => _ = camel_case_4::test(&quiz.input),
        "divisible_sum_pairs" => _ = divisible_sum_pairs::test(&quiz.input),
        "sparse_arrays" => _ = sparse_arrays::test(&quiz.input),
        // WEEK 2
        "lonely_integer" => _ = lonely_integer::test(&quiz.input),
        "grading_students" => _ = grading_students::test(&quiz.input),
        "flipping_bits" => _ = flipping_bits::test(&quiz.input),
        "diagonal_difference" => _ = diagonal_difference::test(&quiz.input),
        "counting_sort_1" => _ = counting_sort_1::test(&quiz.input),
        "counting_valleys" => _ = counting_valleys::test(&quiz.input),
        "pangrams" => _ = pangrams::test(&quiz.input),
        "mars_exploration" => _ = mars_exploration::test(&quiz.input),
        // WEEK 3
        "permuting_two_arrays" => _ = permuting_two_arrays::test(&quiz.input),
        "subarray_division_2" => _ = subarray_division_2::test(&quiz.input),
        "xor_strings_3" => _ = xor_strings_3::test(&quiz.input),
        "sales_by_match" => _ = sales_by_match::test(&quiz.input),
        "migratory_birds" => _ = migratory_birds::test(&quiz.input),
        "maximum_perimeter_triangle" => _ = maximum_perimeter_triangle::test(&quiz.input),
        "zig_zag_sequence" => _ = zig_zag_sequence::test(&quiz.input),
        "drawing_book" => _ = drawing_book::test(&quiz.input),
        _ => println!("Need to add match!"),
    }
}