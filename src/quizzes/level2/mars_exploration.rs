use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let output = QuizOutput {
        name: "mars_exploration".to_string(),
        desc: "String".to_string(),
        example: "String".to_string(),
        level: "level2".to_string(),
        constraints: "String".to_string(),
        input: "String".to_string(),
        output: "String".to_string(),
        output_type: OutputType::VecString,
    };
    output
}

pub fn input_from_file() -> String {
    // load file or panic
    let path = format!(
        "./src/quizzes/{}/{}.txt",
        for_export().level,
        for_export().name
    );
    let input = read_from_input_file(&path);
    input
}

pub fn quiz(arr: &str) -> Vec<i32> {
    let strings = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for str in strings {
        answers.push(marsExploration(&str));
    }
    answers
}

fn read_input(arr: &str) -> Vec<&str> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for line in lines {
        output.push(line);
    }
    output
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
        let answer = vec![3, 1, 0];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
