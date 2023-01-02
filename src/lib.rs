mod quizes;
use crate::quizes::week1::plus_minus;
use crate::quizes::week1::mini_max_sum;
use crate::quizes::week1::time_conversion;

pub fn test(quiz: Quiz) {
    println!("Quiz:\n\t{}", &quiz.name);

    match quiz.name.as_str() {
        "plus_minus" => plus_minus::test(&quiz.input),
        "mini_max_sum" => mini_max_sum::test(&quiz.input),
        "time_conversion" => time_conversion::test(&quiz.input),
        _ => println!("Need to add match!"),
    }
}

pub struct Quiz {
    pub week: String,
    pub name: String,
    pub input: String,
}

impl Quiz {
    pub fn new(path: &String, input: &String, week: &String) -> Quiz {
        let name = path.clone();
        Quiz {
            name,
            week: week.clone(),
            input: input.clone(),
        }
    }
}
