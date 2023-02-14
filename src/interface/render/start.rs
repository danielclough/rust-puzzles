use std::{
    fs,
    io::Error,
    process::{Command, Output, Stdio},
};

use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::interface::{controllers::read::read_quiz_list, types::QuizList};

pub fn test_correct_quiz(file: &str) -> Result<Output, Error> {
    let program = "./target/debug/rq";
    let result = Command::new(program)
        .arg(file)
        .stdout(Stdio::piped())
        .output()?;
    Ok(result)
}
fn get_new_quiz_output(input: &str) -> Result<Output, Error> {
    let result = Command::new("bash")
        .arg("./start.sh")
        .arg(input)
        .stdout(Stdio::piped())
        .output()?;
    Ok(result)
}
fn get_params_from_result(result: Result<Output, Error>) -> String {
    let result_output;
    if result.is_ok() {
        // : Vec<(i32, i32, i32)>
        result_output = String::from_utf8(result.unwrap().stdout).unwrap();
        // assert_eq!(answer, unwrapped);
    } else {
        result_output = format!("{:?}", result.unwrap_err());
    }
    result_output
}

fn compare_results(input_path: &str) -> (String, String, String, bool) {
    // load file or panic
    _ = create_file_if_needed().expect("file should be created");
    let input = fs::read_to_string(input_path).unwrap();
    let input_vec: &Vec<String> = &input.split("\n").map(|x| x.to_string()).collect();
    let input_str = input_vec.join("; ");
    
        let user_result =  get_new_quiz_output(&input_str);
        let user_output = get_params_from_result(user_result);

    let correct_result = test_correct_quiz(input_path);
    let correct_output = get_params_from_result(correct_result);

    (input_str, correct_output.to_owned(), user_output.to_owned(), (correct_output == user_output))
}

pub fn render<'a>(quiz_list_state: &ListState) -> Paragraph<'a> {
    // init state
    let quiz_list: Vec<QuizList> = read_quiz_list().expect("can fetch quiz list");
    let selected_quiz = quiz_list
        .get(
            quiz_list_state
                .selected()
                .expect("there is always a selected quiz"),
        )
        .expect("exists")
        .clone();

    let input_path = &format!("src/quizzes/level{}/{}.txt", selected_quiz.level, selected_quiz.name);
    let comparison_tupl = compare_results(input_path);
    let input_str = &comparison_tupl.0;
    let correct_vec: &Vec<String> = &comparison_tupl.1.split("\n").map(|x| x.to_string()).collect();
    let user_vec: &Vec<String> = &comparison_tupl.2.split("\n").map(|x| x.to_string()).collect();
    let correct_str = correct_vec.join("; ");
    let user_str = user_vec.join("; ");

    // Render Container
    let container = Paragraph::new(vec![
        Spans::from(vec![Span::raw(selected_quiz.example)]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "Constraints: {};",
            selected_quiz.constraints.join("; ")
        ))]),
        Spans::from(vec![Span::raw(format!("Input: {};", input_str))]),
        Spans::from(vec![Span::raw(format!(
            "Output: {};",
            correct_str
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("  User Output:")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(user_str),
        // Spans::from(vec![Span::raw(format!("{:?}",user_vec))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "Correct? {};",
            comparison_tupl.3
        ))]),
    ])
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(format!("Quiz in progress: {} ", selected_quiz.name))
            .border_type(BorderType::Plain),
    );
    container
}

fn create_file_if_needed() -> Result<String, Error> {
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