use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "grading_students".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: "constraints".to_string(),
        level: "level2".to_string(),
        answer: AnswerType::VecVecI32 { answer: vec![vec![75, 67, 40, 33]] },
    };
    output
}


pub fn quiz() -> Vec<Vec<i32>> {
    let ints = read_input();
    let mut answers: Vec<Vec<i32>> = vec![];
    answers.push(gradingStudents(&ints));
    answers
}

fn read_input() -> Vec<i32> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        let int_str: String = line.to_owned().to_owned();
        let int: i32 = int_str.parse::<i32>().expect("num");

        if i != 0 {
            output.push(int);
        }
    }
    output
}

// Input: arr of grades 0..+100
// Rounding Rules:
// (grade - next_round_5) < 3 => round_up_5(grade),
// grade < 38 => do_not_round(grade),
// grade < 40 => fail(grade),
#[allow(non_snake_case)]
fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    let mut final_grades: Vec<i32> = vec![];
    for grade in grades {
        if grade < &38 {
            final_grades.push(*grade);
        } else {
            let rounded = round_up_5(*grade);
            if (rounded - grade) < 3 {
                final_grades.push(rounded);
            } else {
                final_grades.push(*grade);
            }
        }
    }
    for g in &final_grades {
        println!("{:?}", g);
    }
    final_grades
}

fn round_up_5(mut grade: i32) -> i32 {
    while grade % 5 != 0 {
        grade += 1;
    }
    grade
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        assert_eq!(config().answer, AnswerType::VecVecI32 { answer: quiz() } );
    }
}
