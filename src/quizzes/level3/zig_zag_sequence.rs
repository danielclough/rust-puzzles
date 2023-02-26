use crate::quizzes::{types::QuizConfig, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "zig_zag_sequence".to_string(),
        level: "level3".to_string(),
    };
    output
}


// use std::io;
// fn main() {
//     let lines = readlines();
//     test(&lines);
// }
// fn readlines() -> String {
//     use io::prelude::*;
//     let stdin = io::stdin();
//     let v: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
//     v.join("\n")
// }

pub fn quiz() -> Vec<String> {
    let inputs = read_input();
    let mut answers: Vec<String> = vec![];
    for input in inputs {
        answers.push(zig_zag_sequence(input));
    }
    answers
}

#[derive(Clone)]
struct Input {
    // t == n_of_tests
    t: i32,
    // n == lenght of arr_a
    n: i32,
    // a == array of 132
    a: Vec<i32>,
}

fn read_input() -> Vec<Input> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    let mut output: Vec<Input> = vec![
        Input {
            t: 0,
            n: 0,
            a: vec![]
        };
        lines.len() / 3
    ];
    let mut output_i = 0;

    for (i, line) in lines.iter().enumerate() {
        if i % 3 == 0 {
            // t == n_of_tests
            output[output_i].t = line.parse::<i32>().expect("number");
        } else if i % 3 == 1 {
            // n == lenght of arr_a
            output[output_i].n = line.parse::<i32>().expect("number");
        } else if i % 3 == 2 {
            // a == array of 132
            output[output_i].a = line
                .split(" ")
                .map(|x| x.parse::<i32>().expect("number"))
                .collect();
            output_i += 1;
        }
    }
    output
}

fn sort_asc(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if j > j + 1 {
                vec.swap(j, j + 1)
            }
        }
    }
    vec.to_owned()
}
fn sort_dec(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if j < j + 1 {
                vec.swap(j, j + 1)
            }
        }
    }
    vec.to_owned()
}

fn zig_zag_sequence(input: Input) -> String {
    let asc = sort_asc(&mut input.a.to_owned());
    let dec = sort_dec(&mut input.a.to_owned());
    let half1 = input.a.len() / 2;
    let half2 = if input.a.len() % 2 == 0 {
        input.a.len() / 2
    } else {
        (input.a.len() / 2) + 1
    };
    let mut combined = asc[..half1].to_owned();
    combined.append(&mut dec[..half2].to_owned());
    let mut output: Vec<String> = vec![];
    for elem in combined {
        output.push(elem.to_string())
    }
    let output_str = format!("{}", output.join(" "));
    print!("{}", output_str);
    output_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer = vec![String::from("1 2 3 7 6 5 4")];

        assert_eq!(answer, quiz());
    }
}
