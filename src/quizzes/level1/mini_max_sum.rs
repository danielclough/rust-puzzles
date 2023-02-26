use crate::quizzes::{types::QuizConfig, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "mini_max_sum".to_string(),
        level: "level1".to_string(),
    };
    output
}

pub fn quiz(arr: &str) -> Vec<String> {
    let split: Vec<&str> = arr.split("\n").collect();
    let mut answers: Vec<String> = vec![];
    for s in split {
        let str_arr: Vec<&str> = s.split(' ').collect();
        let mut i64_arr: Vec<i64> = vec![];
        for ele in str_arr {
            i64_arr.push(ele.parse::<i64>().expect("number here"));
        }
        answers.push(miniMaxSum(&i64_arr));
    }

    answers
}

#[allow(non_snake_case)]
fn miniMaxSum(arr: &[i64]) -> String {
    let mut sort: Vec<i64> = arr.to_owned();
    sort.sort();
    let mut min = 0;
    let mut max = 0;
    for (i, ele) in &mut sort.iter().enumerate() {
        if i == 0 {
            min += ele;
        } else if i == sort.len() - 1 {
            max += ele;
        } else {
            min += ele;
            max += ele;
        };
    }
    let answer = format!("{} {}", min, max);
    println! {"{}", answer};
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer: Vec<String> = vec![String::from("20 20")];
        let config = config();
        let input = read_from_input_file(&config.level, &config.name);

        assert_eq!(answer, quiz(&input));
    }
}
