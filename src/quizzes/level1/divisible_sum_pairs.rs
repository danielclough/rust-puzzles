use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "6 3
1 3 2 6 1 2";
    let output = QuizOutput {
        name: "divisible_sum_pairs".to_string(),
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

pub fn quiz(arr: &str) -> Vec<i32> {
    let lines: Vec<&str> = arr.split("\n").collect();
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
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
