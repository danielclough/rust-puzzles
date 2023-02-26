use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "diagonal_difference".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: vec!["".to_string()],
        level: "level2".to_string(),
        answer: AnswerType::I32 { answer: 15 },
    };
    output
}


pub fn quiz() -> i32 {
    let ints = read_input();
    diagonalDifference(&ints)
}

fn read_input() -> Vec<Vec<i32>> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
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
        assert_eq!(config().answer, AnswerType::I32 { answer: quiz() } );
    }
}
