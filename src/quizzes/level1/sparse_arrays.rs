use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "6
-4 3 -9 0 4 1";
    let output = QuizOutput {
        name: "sparse_arrays".to_string(),
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

pub fn quiz(arr: &str) -> Vec<Vec<i32>> {
    let input_arr = read_input(arr);
    let mut output: Vec<Vec<i32>> = vec![];
    for input in input_arr {
        output.push(matchingStrings(&input.strings, &input.queries));
    }
    output
}

struct Input {
    strings: Vec<String>,
    queries: Vec<String>,
}

fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    //  var to indicate beginning of series
    let mut left_in_cycle = -1;
    // prepare n and q vars
    // n == ar length (reduce each round)
    let mut n = 0;
    // q == querry length  (reduce each round)
    let mut q = 0;
    // vecs to hold strings and queries
    let mut strings: Vec<String> = vec![];
    let mut queries: Vec<String> = vec![];
    // strut for input
    let mut input_arr: Vec<Input> = vec![];
    for line in lines {
        if left_in_cycle == -1 {
            n = line.parse::<i32>().expect("i32 here");
            left_in_cycle = n;
        } else if n > 0 {
            strings.push(line.to_string());
            n -= 1;
            if n == 0 {
                left_in_cycle = -2;
            }
        } else if left_in_cycle == -2 {
            q = line.parse::<i32>().expect("i32 here");
            left_in_cycle = q;
        } else if q > 0 {
            queries.push(line.to_string());
            q -= 1;
            if q == 0 {
                left_in_cycle = -1;
                // call function at end of cycle
                input_arr.push(Input {
                    strings: strings.to_owned(),
                    queries: queries.to_owned(),
                });
                strings = vec![];
                queries = vec![];
            }
        }
    }
    input_arr
}

#[allow(non_snake_case)]
fn matchingStrings(strings: &[String], queries: &[String]) -> Vec<i32> {
    let length = queries.len();
    let mut return_vec: Vec<i32> = vec![0; length];
    for (i, q) in queries.iter().enumerate() {
        for s in strings {
            if q == s {
                return_vec[i] += 1;
                // println!("{:?}{:?} {} {}",q, s, i, j);
            }
        }
    }
    println!("{:?}", return_vec);
    return_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer: Vec<Vec<i32>> = vec![vec![2, 1, 0], vec![1, 0, 1], vec![1, 3, 4, 3, 2]];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
