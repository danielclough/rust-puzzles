use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let output = QuizOutput {
        name: "permuting_two_arrays".to_string(),
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

pub fn quiz(arr: &str) -> Vec<String> {
    let inputs = read_input(arr);
    let mut answers: Vec<String> = vec![];
    for input in inputs {
        answers.push(twoArrays(input.k, &input.a, &input.b));
    }
    answers
}

#[derive(Clone, Debug)]
struct Input {
    k: i32,
    a: Vec<i32>,
    b: Vec<i32>,
}

enum InputEl {
    N,
    K,
    A,
    B,
}

fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![Input {
        k: 0,
        a: vec![],
        b: vec![],
    }];
    let mut output_n = 0;
    let mut n_left = 0;
    let mut next = InputEl::N;

    for (i, line) in lines.iter().enumerate() {
        match next {
            InputEl::N => {
                n_left = line.parse::<i32>().expect("number");
                next = InputEl::K
            }
            InputEl::K => {
                let n_k: Vec<i32> = line
                    .split(" ")
                    .map(|x| x.parse::<i32>().expect("number"))
                    .collect();
                output[output_n].k = n_k[1];
                next = InputEl::A
            }
            InputEl::A => {
                output[output_n].a = line
                    .split(" ")
                    .map(|x| x.parse::<i32>().expect("number"))
                    .collect();
                next = InputEl::B
            }
            InputEl::B => {
                output[output_n].b = line
                    .split(" ")
                    .map(|x| x.parse::<i32>().expect("number"))
                    .collect();
                n_left -= 1;
                if n_left != 0 {
                    next = InputEl::K
                } else {
                    next = InputEl::N;
                }
                if i < lines.len() - 1 {
                    output_n += 1;
                    output.push(Input {
                        k: 0,
                        a: vec![],
                        b: vec![],
                    });
                }
            }
        }

        println!(
            "n_left:{}\t o_n:{}\n line:{:?}\n\n",
            n_left, output_n, output
        );
    }
    output
}

fn sort_asc(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1)
            }
        }
    }
}
fn sort_dec(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j + 1] > arr[j] {
                arr.swap(j + 1, j)
            }
        }
    }
}

#[allow(non_snake_case)]
fn twoArrays(k: i32, A: &[i32], B: &[i32]) -> String {
    let mut answer_vec: Vec<&str> = vec![];
    // order vec1 asc
    let mut asc: Vec<i32> = A.to_vec();
    sort_asc(&mut asc);
    // order vec2 dec
    let mut dec: Vec<i32> = B.to_vec();
    sort_dec(&mut dec);
    // add vecs el by el IF any sum < k NO
    for (i, el) in asc.iter().enumerate() {
        if el + dec[i] >= k {
            answer_vec.push("YES");
        } else {
            answer_vec.push("NO");
        }
    }

    // else YES

    let mut answer: String = String::new();
    if answer_vec.contains(&"NO") {
        answer.push_str("NO");
    } else {
        answer.push_str("YES");
    };
    println!("{} {:?} {:?}", answer, asc, dec);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn does_it_work() {
        let answer = vec!["YES".to_owned(), "NO".to_owned(), "NO".to_owned()];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
