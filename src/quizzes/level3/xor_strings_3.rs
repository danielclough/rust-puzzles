use crate::quizzes::{types::QuizConfig, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "xor_strings_3".to_string(),
        level: "level3".to_string(),
    };
    output
}


// use std::io;
// fn main() {
//     let lines = readlines();
//     let mut output: Vec<Vec<&str>> = vec![Vec::new()];
//     let mut output_n = 0;

//     for (i, line) in lines.iter().enumerate() {
//         if i < line.len() {
//             output[output_n].push(line);
//         }
//         if i % 2 == 1 {
//             output_n += 1;
//         }
//     }
//     for input in output {
//         xor_strings_3(input);
//     }
// }
// fn readlines() -> Vec<String> {
//     use io::prelude::*;
//     let stdin = io::stdin();
//     let v = stdin.lock().lines().map(|x| x.unwrap()).collect();
//     v
// }

pub fn quiz(arr: &str) -> Vec<String> {
    let inputs = read_input(arr);
    let mut answers: Vec<String> = vec![];
    for input in inputs {
        answers.push(xor_strings_3(input));
    }
    answers
}

fn read_input(arr: &str) -> Vec<Vec<&str>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output: Vec<Vec<&str>> = vec![Vec::new()];
    let mut output_n = 0;

    for (i, line) in lines.iter().enumerate() {
        if i < line.len() {
            output[output_n].push(line);
        }
        if i % 2 == 1 {
            output_n += 1;
        }
    }
    output
}

fn xor_strings_3(input: Vec<&str>) -> String {
    let mut collector: Vec<i32> = vec![];
    let v0: Vec<i32> = input[0].chars().map(|x| x as i32).collect();
    let v1: Vec<i32> = input[1].chars().map(|x| x as i32).collect();
    for i in 0..input[0].len() {
        collector.push(v0[i] ^ v1[i]);
    }
    let answer = format!(
        "{}",
        collector
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    println!("{}", answer);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer = vec!["10000"];
        let config = config();
        let input = read_from_input_file(&config.level, &config.name);

        assert_eq!(answer, quiz(&input));
    }
}
