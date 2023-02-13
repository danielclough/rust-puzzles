use std::{
    fs,
    io::Error,
    process::{Command, Output, Stdio},
};

use rand::Rng;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::interface::{controllers::read::read_quiz_list, types::QuizList};

pub fn test_quiz(file: &str) -> Result<Output, Error> {
    let mut rng = rand::thread_rng();
    let rand: [f32; 6] = rng.gen();
    let program = "./target/debug/rq";
    let result = Command::new(program)
        .arg(file)
        .stdout(Stdio::piped())
        .output()?;
    Ok(result)
}
fn get_quiz_params(path: &str) -> (String, String) {
    // load file or panic
    let input = fs::read_to_string(path).unwrap();

    let result = test_quiz(path);
    let result_output;
    if result.is_ok() {
        // : Vec<(i32, i32, i32)>
        result_output = String::from_utf8(result.unwrap().stdout).unwrap();
        // assert_eq!(answer, unwrapped);
    } else {
        result_output = format!("{:?}", result.err());
    }
    (input, result_output)
}
fn compare_results() -> (String, String) {
    let path = "src/quizzes/level1/plus_minus.txt";
    let result_output = get_quiz_params(path);
    result_output
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

    let result_output = compare_results();
    let output_vec: &Vec<String> = &result_output.1.split("\n").map(|x| x.to_string()).collect();
    let input_vec: &Vec<String> = &result_output.0.split("\n").map(|x| x.to_string()).collect();
    let output_str = output_vec.join("; ");
    let input_str = input_vec.join("; ");

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
            selected_quiz.output.join("; ")
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("User Output:")]),
        Spans::from(output_str),
        // Spans::from(vec![Span::raw(format!("{:?}",result))]),
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
