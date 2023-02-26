use crate::quizzes::{types::QuizConfig, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "divisible_sum_pairs".to_string(),
        level: "level1".to_string(),
    };
    output
}


pub fn quiz() -> Vec<i32> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    let mut answers: Vec<i32> = vec![];
    // prepare n and k vars
    // n == ar length
    let mut n = 0;
    // k == divisor
    let mut k = 0;

    for (i, line) in lines.iter().enumerate() {
        if i % 2 == 0 {
            let str_arr: Vec<&str> = line.split(' ').collect();
            n = str_arr[0].parse::<i32>().expect("number here");
            k = str_arr[1].parse::<i32>().expect("number here");
        }
        // prepare ar and call function
        if i % 2 == 1 {
            let mut ar: Vec<i32> = vec![];
            let arr: Vec<&str> = line.split(' ').collect();

            for a in arr {
                ar.push(a.parse::<i32>().expect("number here"))
            }
            answers.push(divisibleSumPairs(n, k, &ar));
        }
    }
    answers
}

#[allow(non_snake_case)]
fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    // determine the number of (i,j) pairs where i<j and ar[i] + ar[j] is divisible by k.
    // pairs == number of pairs
    let _junk = n;
    let mut pairs = 0;
    for (i, a) in ar.iter().enumerate() {
        for (j, b) in ar.iter().enumerate() {
            if j > i && (a + b) % k == 0 {
                pairs += 1
            }
        }
    }
    println!("{}", pairs);
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer: Vec<i32> = vec![5];

        assert_eq!(answer, quiz());
    }
}
