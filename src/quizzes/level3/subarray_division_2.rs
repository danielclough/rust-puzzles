use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let output = QuizOutput {
        name: "subarray_division_2".to_string(),
        desc: "String".to_string(),
        example: "String".to_string(),
        level: "level3".to_string(),
        constraints: "String".to_string(),
        input: "String".to_string(),
        output: "String".to_string(),
        output_type: OutputType::VecString,
    };
    output
}

pub fn input_from_file() -> String {
    // load file or panic
    let path = format!(
        "./src/quizzes/{}/{}.txt",
        for_export().level,
        for_export().name
    );
    let input = read_from_input_file(&path);
    input
}

pub fn quiz(arr: &str) -> Vec<i32> {
    let inputs = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for input in inputs {
        answers.push(birthday(&input.ar, input.d, input.m));
    }
    answers
}

#[derive(Clone)]
struct Input {
    ar: Vec<i32>,
    d: i32,
    m: i32,
}
impl Input {
    fn new() -> Input {
        Input {
            ar: vec![],
            d: 0,
            m: 0,
        }
    }
}

fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output: Vec<Input> = vec![Input::new()];
    let mut output_n = 0;

    for (i, line) in lines.iter().enumerate() {
        if i % 3 == 1 {
            output[output_n].ar = line
                .split(" ")
                .map(|x| x.parse::<i32>().expect("number"))
                .collect();
        } else if i % 3 == 2 {
            let tmp: Vec<i32> = line
                .split(" ")
                .map(|x| x.parse::<i32>().expect("number"))
                .collect();
            output[output_n].d = tmp[0];
            output[output_n].m = tmp[1];
            output_n += 1;
            if i < lines.len() - 1 {
                output.push(Input::new());
            }
        }
    }
    output
}

// line1 == n squares of chocolate bar
// line2 == i32_arr
// line3 == d (birth day) and m (birth month)

// length of segment == birth month
// sum of ints on squares == birth day.

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut answer = 0;
    println!("{} {} {:?}", d, m, s);
    // divide s into parts with m length
    for i in 0..s.len() {
        let mut collector: Vec<i32> = vec![];
        let n: usize = i + m as usize;
        if n <= s.len() {
            for j in i..n {
                collector.push(s[j] as i32);
            }
            let mut sum = 0;
            for k in collector {
                sum += k;
            }
            // IF sum of parts == d => answer++
            if sum == d {
                answer += 1;
            }
        }
    }
    println!("{:?}", answer);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn does_it_work() {
        let answer = vec![2, 0, 1];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
