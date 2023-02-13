use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let output = QuizOutput {
        name: "grading_students".to_string(),
        desc: "String".to_string(),
        example: "String".to_string(),
        level: "level2".to_string(),
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

pub fn quiz(arr: &str) -> Vec<Vec<i32>> {
    let ints = read_input(arr);
    let mut answers: Vec<Vec<i32>> = vec![];
    answers.push(gradingStudents(&ints));
    answers
}

fn read_input(arr: &str) -> Vec<i32> {
    let lines: Vec<&str> = arr.split("\n").collect();
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
        let answer = vec![vec![75, 67, 40, 33]];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
