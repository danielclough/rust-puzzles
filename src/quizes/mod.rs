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
// use crate::quizes::week2::counting_valleys;
// use crate::quizes::week2::pangrams;
// use crate::quizes::week2::mars_exploration;

pub fn match_quiz(quiz: Quiz) {
    println!("Quiz:\n\t{}\n", &quiz.name);

    match quiz.name.as_str() {
        "plus_minus" => _ = plus_minus::test(&quiz.input),
        "mini_max_sum" => _ = mini_max_sum::test(&quiz.input),
        "time_conversion" => _ = time_conversion::test(&quiz.input),
        "breaking_the_records" => _ = breaking_the_records::test(&quiz.input),
        "camel_case_4" => _ = camel_case_4::test(&quiz.input),
        "divisible_sum_pairs" => _ = divisible_sum_pairs::test(&quiz.input),
        "sparse_arrays" => _ = sparse_arrays::test(&quiz.input),
        "lonely_integer" => _ = lonely_integer::test(&quiz.input),
        "grading_students" => _ = grading_students::test(&quiz.input),
        "flipping_bits" => _ = flipping_bits::test(&quiz.input),
        "diagonal_difference" => _ = diagonal_difference::test(&quiz.input),
        "counting_sort_1" => _ = counting_sort_1::test(&quiz.input),
        // "counting_valleys" => _ = counting_valleys::test(&quiz.input),
        // "pangrams" => _ = pangrams::test(&quiz.input),
        // "mars_exploration" => _ = mars_exploration::test(&quiz.input),
        _ => println!("Need to add match!"),
    }
}