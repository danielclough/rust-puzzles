use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "sales_by_match".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: "constraints".to_string(),
        level: "level3".to_string(),
        answer: AnswerType::VecI32 { answer: vec![3, 4] },
    };
    output
}


pub fn quiz() -> Vec<i32> {
    let inputs = read_input();
    let mut answers: Vec<i32> = vec![];
    for input in inputs {
        answers.push(sockMerchant(input.n, &input.ar));
    }
    answers
}

#[derive(Clone)]
struct Input {
    n: i32,
    ar: Vec<i32>,
}

fn read_input() -> Vec<Input> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    let mut output = vec![Input { n: 0, ar: vec![] }; lines.len() / 2];
    let mut output_n = 0;

    for (i, line) in lines.iter().enumerate() {
        if i % 2 == 0 {
            output[output_n].n = line.parse::<i32>().expect("number");
        } else if i % 2 == 1 {
            output[output_n].ar = line
                .split(" ")
                .map(|x| x.parse::<i32>().expect("number"))
                .collect();
            output_n += 1;
        }
    }
    output
}

#[derive(Debug)]
struct SockType {
    id: i32,
    found: i32,
}

#[allow(non_snake_case)]
fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    _ = n;
    let mut socktypes: Vec<SockType> = Vec::new();
    for a in ar {
        let mut counter = 0;
        for (_, s) in socktypes.iter_mut().enumerate() {
            println!("{:?} {:?}", a, s.id);
            if &s.id == a {
                s.found += 1;
            } else {
                counter += 1;
            };
        }

        if counter == socktypes.len() || socktypes.len() == 0 {
            socktypes.push(SockType { id: *a, found: 1 });
        }
    }
    let mut pairs = 0;
    for s in socktypes {
        pairs += s.found / 2;
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        assert_eq!(config().answer, AnswerType::VecI32 { answer: quiz() } );
    }
}
