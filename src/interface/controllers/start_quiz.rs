use tui::widgets::ListState;

use crate::interface::types::{MenuConfig, MenuItem, Comparison, QuizList};
use std::{
    fs,
    io::Error,
    process::{Command, Output, Stdio}, time::{SystemTime, UNIX_EPOCH},
};

use super::read::read_quiz_list;

pub fn exec(menu_config: &MenuConfig) {
    match menu_config.active_item {
        MenuItem::Home => {}
        _ => {}
    }
}

pub fn get_elapsed() -> String {
        // create timestamp and store it
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp: String;
    let timestamp_path = "./user-data/.timestamp";
    let read_timestamp = std::fs::read_to_string(timestamp_path);
    if read_timestamp.is_ok() {
        timestamp = read_timestamp.unwrap()
    } else {
        timestamp = format!("{}",duration_since_epoch.as_secs());
        _ = std::fs::write(timestamp_path, &timestamp);
    }

    let elapsed_in_sec = duration_since_epoch.as_secs() as i64 - timestamp.parse::<i64>().unwrap();
    let elapsed_hrs = elapsed_in_sec / (60*60);
    let elapsed_mins = elapsed_in_sec % (60*60) / 60;
    let elapsed_secs = elapsed_in_sec % 60;
    let elapsed = format!("{}:{}:{}", elapsed_hrs, elapsed_mins, elapsed_secs);
    elapsed
}

pub fn init_compare(selected_quiz: &QuizList) -> Comparison {
    let input_path = &format!(
        "src/quizzes/level{}/{}.txt",
        selected_quiz.level, selected_quiz.name
    );
    let comparison_tupl = compare_results(input_path);
    let input_str = &comparison_tupl.0;
    let correct_vec: &Vec<String> = &comparison_tupl
        .1
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    let user_vec: &Vec<String> = &comparison_tupl
        .2
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    let correct_tmp = correct_vec.join("; ");
    let correct_str = correct_tmp.trim();
    let user_tmp = user_vec.join("; ");
    let user_str = user_tmp.trim();
    let is_correct = comparison_tupl.3;

    let elapsed = get_elapsed();

    let comparison = Comparison {
        input_str: input_str.to_string(),
        correct_str: correct_str.to_string(),
        user_str: user_str.to_string(),
        is_correct,
        elapsed,
    };
    comparison
}

pub fn test_correct_quiz(file: &str) -> Result<Output, Error> {
    let program = "./target/debug/rq";
    let result = Command::new(program)
        .arg(file)
        .stdout(Stdio::piped())
        .output()?;
    Ok(result)
}
fn compile_new_quiz() {
    let program = "./build.sh";
    _ = Command::new(program)
        .stdout(Stdio::piped())
        .output();
}
fn get_new_quiz_output(input: &str) -> Result<Output, Error> {
    _ = compile_new_quiz();
    
    let program = "./user-data/target/debug/user-data";
    let result = Command::new(program)
        .arg(input)
        .stdout(Stdio::piped())
        .output()?;
    Ok(result)
}
fn get_params_from_result(result: Result<Output, Error>) -> String {
    let result_output;
    if result.is_ok() {
        result_output = String::from_utf8(result.unwrap().stdout).unwrap();
    } else {
        result_output = format!("{:?}", result.unwrap_err());
    }
    result_output
}

pub fn compare_results(input_path: &str) -> (String, String, String, bool) {
    // load file or panic
    let input = fs::read_to_string(input_path).unwrap();
    let input_vec: &Vec<String> = &input.split("\n").map(|x| x.to_string()).collect();
    let input_str = input_vec.join("; ");

    let user_result = get_new_quiz_output(&input_str);
    let user_output = get_params_from_result(user_result);

    let correct_result = test_correct_quiz(input_path);
    let correct_output = get_params_from_result(correct_result);

    (
        input_str,
        correct_output.to_owned(),
        user_output.to_owned(),
        correct_output.eq(&user_output),
    )
}

pub fn create_file_if_needed() -> Result<String, Error> {
    let new_quiz_path = "./user-data/src/main.rs";
    if fs::read(new_quiz_path).is_err() {
        let new_quiz_content = "use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input: Vec<String> = args[1].to_owned().split(\"; \").map(|x| x.to_string()).collect();
        println!(\"Test Input: {:?}\", input);
    }
}";
        fs::write(new_quiz_path, new_quiz_content)?;
    };
    Ok(new_quiz_path.to_string())
}


pub fn get_selected_quiz(quiz_list_state: &ListState) -> QuizList {
    let quiz_list: Vec<QuizList> = read_quiz_list().expect("can fetch quiz list");
    quiz_list
        .get(
            quiz_list_state
                .selected()
                .expect("there is always a selected quiz"),
        )
        .expect("exists")
        .clone()
}