use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "lonely_integer".to_string(),

        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: "constraints".to_string(),
        level: "level2".to_string(),
        answer: AnswerType::VecI32 { answer: vec![1, 2, 2] },
    };
    output
}

pub fn quiz() -> Vec<i32> {
    let ints = read_input();
    let mut answers: Vec<i32> = vec![];
    for arr in ints {
        answers.push(lonelyinteger(&arr));
    }
    answers
}

fn read_input() -> Vec<Vec<i32>> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
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
        assert_eq!(config().answer, AnswerType::VecI32 { answer: quiz() } );
    }
}
