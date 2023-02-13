use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "S;M;plasticCup()
C;V;mobile phone
C;C;coffee machine
S;C;LargeSoftwareBook
C;M;white sheet of paper
S;V;pictureFrame";
    let output = QuizOutput {
        name: "plus_minus".to_string(),
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

pub fn quiz(arr: &str) -> Vec<(f32, f32, f32)> {
    let split: Vec<&str> = arr.split("\n").collect();
    let mut answers: Vec<(f32, f32, f32)> = vec![];

    let str_arr: Vec<&str> = split[1].split(' ').collect();
    let mut i32_arr: Vec<i32> = vec![];
    for ele in str_arr {
        i32_arr.push(ele.parse::<i32>().expect("number here"));
    }
    answers.push(plusMinus(&i32_arr));

    answers
}

#[allow(non_snake_case)]
fn plusMinus(arr: &[i32]) -> (f32, f32, f32) {
    let n = &arr.len();

    // get ratio of pos/n, neg/n, zer/n
    let (mut pos, mut neg, mut zer) = (0, 0, 0);
    for ele in arr {
        if ele > &0 {
            pos += 1;
        } else if ele < &0 {
            neg += 1;
        } else {
            zer += 1;
        }
    }
    let pos_str = format!("{0:.6}", pos as f32 / *n as f32);
    let neg_str = format!("{0:.6}", neg as f32 / *n as f32);
    let zer_str = format!("{0:.6}", zer as f32 / *n as f32);
    println!("{}", pos_str);
    println!("{}", neg_str);
    println!("{}", zer_str);

    let answer = (
        pos_str.parse::<f32>().expect("f32"),
        neg_str.parse::<f32>().expect("f32"),
        zer_str.parse::<f32>().expect("f32"),
    );
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer: Vec<(f32, f32, f32)> = vec![(0.500000, 0.333333, 0.166667)];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
