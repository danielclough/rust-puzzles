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