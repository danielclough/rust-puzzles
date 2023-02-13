use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "";
    let output = QuizOutput {
        name: "lonely_integer".to_string(),
        desc: "String".to_string(),
        example: "String".to_string(),
        level: "level2".to_string(),
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
    let ints = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for arr in ints {
        answers.push(lonelyinteger(&arr));
    }
    answers
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        let str_arr: Vec<&str> = line.split(' ').collect();
        let mut i32_arr: Vec<i32> = vec![];
        for ele in str_arr {
            i32_arr.push(ele.parse::<i32>().expect("number here"));
        }

        if i % 2 == 1 {
            output.push(i32_arr);
        }
    }
    output
}
#[derive(PartialEq, Debug, Clone, Copy)]
struct Has {
    int: i32,
    n: i32,
}

// Input == int arr
// Output == unique int in arr
fn lonelyinteger(a: &[i32]) -> i32 {
    let mut a_has: Vec<Has> = vec![];
    for int_from_a in a {
        if a_has.contains(&Has {
            int: int_from_a.to_owned(),
            n: 1,
        }) {
            let index = a_has
                .iter()
                .position(|x| x.int == int_from_a.to_owned())
                .unwrap();
            a_has[index].n += 1;
            println!("Index: {}", index);
        } else {
            a_has.push(Has {
                int: int_from_a.to_owned(),
                n: 1,
            });
            println!("a_has: {:?}", a_has);
        }
    }
    let answer_index = a_has.iter().position(|x| x.n == 1).unwrap();
    let answer: Has = a_has[answer_index];
    answer.int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer = vec![1, 2, 2];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
