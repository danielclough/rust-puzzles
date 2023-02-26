use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "subarray_division_2".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: "constraints".to_string(),
        level: "level3".to_string(),
        answer: AnswerType::VecI32 { answer: vec![2, 0, 1] },
    };
    output
}


pub fn quiz() -> Vec<i32> {
    let inputs = read_input();
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

fn read_input() -> Vec<Input> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
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
        assert_eq!(config().answer, AnswerType::VecI32 { answer: quiz() } );
    }
}
