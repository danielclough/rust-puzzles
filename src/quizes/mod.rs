use crate::Quiz;
use crate::quizes::week1::plus_minus;
use crate::quizes::week1::mini_max_sum;
use crate::quizes::week1::time_conversion;
use crate::quizes::week1::breaking_the_records;
use crate::quizes::week1::camel_case_4;
use crate::quizes::week1::divisible_sum_pairs;
use crate::quizes::week1::sparse_arrays;
// use crate::quizes::week1::

pub mod week1;

pub fn match_quiz(quiz: Quiz) {
    println!("Quiz:\n\t{}\n", &quiz.name);

    match quiz.name.as_str() {
        "plus_minus" => plus_minus::test(&quiz.input),
        "mini_max_sum" => mini_max_sum::test(&quiz.input),
        "time_conversion" => time_conversion::test(&quiz.input),
        "breaking_the_records" => breaking_the_records::test(&quiz.input),
        "camel_case_4" => camel_case_4::test(&quiz.input),
        "divisible_sum_pairs" => divisible_sum_pairs::test(&quiz.input),
        "sparse_arrays" => sparse_arrays::test(&quiz.input),
        // "" => ::test(&quiz.input),
        _ => println!("Need to add match!"),
    }
}