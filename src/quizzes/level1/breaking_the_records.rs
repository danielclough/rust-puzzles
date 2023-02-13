use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "9
10 5 20 20 4 5 2 25 1";
    let output = QuizOutput {
        name: "breaking_the_records".to_string(),
        desc: "String".to_string(),
        example: "String".to_string(),
        level: "level1".to_string(),
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

pub fn quiz(arr: &str) -> Vec<Vec<i32>> {
    let scores = read_input(arr);
    let mut answers: Vec<Vec<i32>> = vec![];
    for s in scores {
        answers.push(breakingRecords(&s));
    }
    answers
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        let str_arr: Vec<&str> = line.split(' ').collect();
        let mut i32_arr: Vec<i32> = vec![];
        for ele in str_arr {
            i32_arr.push(ele.parse::<i32>().expect("number here"));
        }

        if i % 2 == 1 {
            output.push(i32_arr);
        }
    }
    output
}

#[allow(non_snake_case)]
fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    // let n = &scores.len();
    let mut current_min = 0;
    let mut min = 0;
    let mut current_max = 0;
    let mut max = 0;
    for (i, score) in scores.iter().enumerate() {
        if i == 0 {
            current_min = score.to_owned();
            current_max = score.to_owned();
        } else if score < &current_min {
            min += 1;
            current_min = score.to_owned();
            println!("min: {} {} {}", score, min, current_min);
        } else if score > &current_max {
            max += 1;
            current_max = score.to_owned();
            println!("max: {} {} {}", score, max, current_max);
        }
    }
    let mut return_vec: Vec<i32> = vec![];
    return_vec.push(max);
    return_vec.push(min);

    println!("{} {}", max, min);
    println!("{:?}", return_vec);
    return_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer: Vec<Vec<i32>> = vec![vec![2, 4], vec![4, 0]];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
