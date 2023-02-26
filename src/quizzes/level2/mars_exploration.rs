use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "mars_exploration".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: "constraints".to_string(),
        level: "level2".to_string(),
        answer: AnswerType::VecI32 { answer: vec![3, 1, 0] },
    };
    output
}

pub fn quiz() -> Vec<i32> {
    let strings = read_input();
    let mut answers: Vec<i32> = vec![];
    for str in strings {
        answers.push(marsExploration(&str));
    }
    answers
}

fn read_input() -> Vec<String> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    let mut output = vec![];

    for line in lines {
        output.push(line.to_string());
    }
    output.to_owned()
}

#[allow(non_snake_case)]
fn marsExploration(s: &str) -> i32 {
    let str_arr: Vec<&str> = s.split("").collect();
    let mut deranged = 0;
    for (i, str) in str_arr.iter().filter(|&x| !x.is_empty()).enumerate() {
        if i % 3 == 0 || i % 3 == 2 {
            if str != &"S" {
                deranged += 1;
                println!("NotS {}", str);
            }
        } else if i % 3 == 1 {
            if str != &"O" {
                deranged += 1;
                println!("NotO {}", str);
            }
        }
    }
    deranged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        assert_eq!(config().answer, AnswerType::VecI32 { answer: quiz() } );
    }
}
