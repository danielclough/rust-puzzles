mod quizes;
use crate::quizes::plus_minus;
use crate::quizes::mini_max_sum;

pub fn test(quiz: Quiz) {
    println!("Quiz:\n\t{}", &quiz.name);

    match quiz.name.as_str() {
        "plus_minus" => plus_minus::test(&quiz.input),
        "mini_max_sum" => mini_max_sum::test(&quiz.input),
        _ => println!("Need to add match!"),
    }
}


pub struct Quiz {
    pub name: String,
    pub input: String,
}

impl Quiz {
    pub fn new(path: &String, input: &String) -> Quiz {
        let name = path.clone();
        Quiz {
            name,
            input: input.clone(),
        }
    }
}
