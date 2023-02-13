use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "3
11 2 4
4 5 6
10 8 -12";
    let output = QuizOutput {
        name: "diagonal_difference".to_string(),
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

pub fn quiz(arr: &str) -> i32 {
    let ints = read_input(arr);
    diagonalDifference(&ints)
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

        if i != 0 {
            output.push(i32_arr);
        }
    }
    output
}

#[allow(non_snake_case)]
fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    // println!("{:?}", arr);
    let mut primary_diag = 0;
    let mut secondary_diag = 0;
    for (i, a) in arr.iter().enumerate() {
        primary_diag = primary_diag + a[i];
        secondary_diag = secondary_diag + a[a.len() - i - 1];
    }
    let output = secondary_diag - primary_diag;
    output.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer = 15;
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
