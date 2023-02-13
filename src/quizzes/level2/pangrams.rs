use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "We promptly judged antique ivory buckles for the next prize";
    let output = QuizOutput {
        name: "pangrams".to_string(),
        desc: "String".to_string(),
        example: "String".to_string(),
        level: "level2".to_string(),
        constraints: "String".to_string(),
        input: format!("{:?}", input),
        output: "String".to_string(),
        output_type: OutputType::VecString,
    };
    output
}

pub fn input_from_file() -> String {
    let for_export = for_export();
    // load file or panic
    let path = format!("./src/quizzes/{}/{}.txt", for_export.level, for_export.name);
    let input = read_from_input_file(&path);
    input
}

pub fn quiz(arr: &str) -> Vec<String> {
    let strings = read_input(arr);
    let mut answers: Vec<String> = vec![];
    for string in strings {
        answers.push(pangrams(&string));
    }
    answers
}

fn read_input(arr: &str) -> Vec<String> {
    let lines: Vec<&str> = arr.split("\n").collect();
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
        let answer = vec![String::from("pangram"), String::from("not pangram")];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
