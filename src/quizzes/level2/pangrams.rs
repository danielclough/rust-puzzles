use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "pangrams".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: "constraints".to_string(),
        level: "level2".to_string(),
        answer: AnswerType::VecString { answer: vec![String::from("pangram"), String::from("not pangram")] },
    };
    output
}


pub fn quiz() -> Vec<String> {
    let strings = read_input();
    let mut answers: Vec<String> = vec![];
    for string in strings {
        answers.push(pangrams(&string));
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
    output
}

fn pangrams(s: &str) -> String {
    let alphabet: Vec<&str> = "abcdefghijklmnopqrstuvwxyz".split("").collect();
    let mut count_arr = vec![0; alphabet.len()];
    let str_arr: Vec<&str> = s.split("").collect();
    let mut answer: String = String::from("pangram");
    for str in str_arr {
        for (i, alpha) in alphabet.iter().map(|x| x.to_owned()).enumerate() {
            if str.to_lowercase() == alpha {
                count_arr[i] += 1;
            };
        }
    }
    for c in &count_arr {
        if c == &0 {
            answer = String::from("not pangram");
            break;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        assert_eq!(config().answer, AnswerType::VecString { answer: quiz() } );
    }
}
