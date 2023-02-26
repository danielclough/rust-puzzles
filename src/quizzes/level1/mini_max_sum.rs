use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "mini_max_sum".to_string(),
        desc: "Find the min and max values by summing four of five given integers.".to_string(),
        example: "Print the min and max values as a single line separated by a space.".to_string(),
        constraints: vec!["1 <= arr[i] <= 10^9".to_string()],
        level: "level1".to_string(),
        answer: AnswerType::VecString { answer: vec![
            String::from("20 20"),
            String::from("166 236")
            ] },
    };
    output
}

pub fn quiz() -> Vec<String> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    let mut answers: Vec<String> = vec![];
    for s in lines {
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
        assert_eq!(config().answer, AnswerType::VecString { answer: quiz() } );
    }
}
