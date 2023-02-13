use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "10101
00101";
    let output = QuizOutput {
        name: "xor_strings_3".to_string(),
        desc: "String".to_string(),
        example: "String".to_string(),
        level: "level3".to_string(),
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
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
