use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "drawing_book".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: "constraints".to_string(),
        level: "level3".to_string(),
        answer: AnswerType::VecI32 { answer: vec![1, 0, 1, 1] },
    };
    output
}


pub fn quiz() -> Vec<i32> {
    let inputs = read_input();
    let mut answers: Vec<i32> = vec![];
    for input in inputs {
        answers.push(pageCount(input.n, input.p));
    }
    answers
}
#[derive(Clone)]
struct Input {
    //  number of pages in the book
    n: i32,
    //  page to turn to
    p: i32,
}

fn read_input() -> Vec<Input> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    let mut output: Vec<Input> = vec![Input { n: 0, p: 0 }; lines.len() / 2];
    let mut output_i = 0;
    for (i, line) in lines.iter().enumerate() {
        let int32 = line.parse::<i32>().expect("number");
        if i % 2 == 0 {
            output[output_i].n = int32;
        } else if i % 2 == 1 {
            output[output_i].p = int32;
            output_i += 1;
        }
    }
    output
}

// book is a zero indexed array
// Return min number of pages to flip (one at a time) to page p
// book is n pages long
// start at beggining or end
// counting starts at 1 or n
#[allow(non_snake_case)]
fn pageCount(n: i32, p: i32) -> i32 {
    let book_length = n;
    let start_from_back = if book_length as f32 - p as f32 >= book_length as f32 / 2 as f32 {
        false
    } else {
        true
    };

    let mut current = (0, 1);
    let mut count = 0;
    if start_from_back {
        // weirdness for page only starts on front rule
        if book_length % 2 == 0 {
            // even pages start on black back
            current = (book_length, book_length + 1);
        } else {
            // odd pages start on two readable pages
            current = (book_length - 1, book_length);
        }
        while current.0 != p && current.1 != p {
            current = (current.0 - 2, current.1 - 2);
            count += 1;
        }
    } else {
        while current.0 != p && current.1 != p {
            current = (current.0 + 2, current.1 + 2);
            count += 1;
        }
    }

    println!("{:?} {} {} {}", current, count, n, p);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        assert_eq!(config().answer, AnswerType::VecI32 { answer: quiz() } );
    }
}
