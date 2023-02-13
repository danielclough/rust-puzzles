use crate::quizzes::{
    types::{OutputType, QuizOutput},
    utils::read_from_input_file,
};

pub fn for_export() -> QuizOutput {
    let input = "07:05:45AM";
    let output = QuizOutput {
        name: "time_conversion".to_string(),
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
    let path = format!(
        "./src/quizzes/{}/{}.txt",
        for_export.level,
        for_export.name
    );
    let input = read_from_input_file(&path);
    input
}

pub fn quiz(arr: &str) -> Vec<String> {
    let split: Vec<&str> = arr.split("\n").collect();
    let mut answers: Vec<String> = vec![];

    for ele in split {
        answers.push(timeConversion(&ele))
    }
    answers
}

#[allow(non_snake_case)]
fn timeConversion(s: &str) -> String {
    let time_split: Vec<&str> = s.split(':').collect();
    let hours_i32: i32;
    let mut hours: String;
    let minutes = time_split[1];
    let seconds_str: String;
    if time_split[2].contains("AM") {
        hours_i32 = time_split[0].parse::<i32>().unwrap().to_owned();
        if hours_i32 == 12 || hours_i32 == 0 {
            hours = String::from("00");
        } else {
            hours = hours_i32.to_string();
            if hours.len() == 1 {
                hours = format!("{}{}", '0', hours)
            }
        }

        seconds_str = time_split[2][0..2].parse::<String>().unwrap();
    } else {
        hours_i32 = time_split[0].parse::<i32>().unwrap().to_owned() + 12;
        if hours_i32 == 24 {
            hours = String::from("12");
        } else {
            hours = hours_i32.to_string();
            if hours.len() == 1 {
                hours = format!("{}{}", '0', hours)
            }
        }

        seconds_str = time_split[2][0..2].parse::<String>().unwrap();
    }
    let time_str = format!("{hours}:{minutes}:{seconds_str}");
    println!("{:?}", time_str);
    time_str
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn does_it_work() {
        let answer: Vec<String> = vec![
            String::from("07:05:45"),
            String::from("19:05:45"),
            String::from("00:40:22"),
            String::from("12:40:22"),
            String::from("00:00:01"),
            String::from("12:00:01"),
        ];
        let input = input_from_file();

        assert_eq!(answer, quiz(&input));
    }
}
