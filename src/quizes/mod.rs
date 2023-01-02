use crate::Quiz;
use crate::quizes::week1::plus_minus;
use crate::quizes::week1::mini_max_sum;
use crate::quizes::week1::time_conversion;
use crate::quizes::week1::breaking_the_records;
// use crate::quizes::week1::

pub mod week1;

pub fn match_quiz(quiz: Quiz) {
    println!("Quiz:\n\t{}\n", &quiz.name);

    match quiz.name.as_str() {
        "plus_minus" => plus_minus::test(&quiz.input),
        "mini_max_sum" => mini_max_sum::test(&quiz.input),
        "time_conversion" => time_conversion::test(&quiz.input),
        "breaking_the_records" => breaking_the_records::test(&quiz.input),
        // "" => ::test(&quiz.input),
        _ => println!("Need to add match!"),
    }
}