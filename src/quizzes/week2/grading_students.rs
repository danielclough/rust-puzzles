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
    use std::fs;

    #[test]
    fn does_it_work() {
        let answer = vec![vec![75, 67, 40, 33]];

        // load file or panic
        let path = "input/week2/grading_students.txt";
        let input = fs::read_to_string(path).unwrap();

        assert_eq!(answer, quiz(&input));
    }
}
