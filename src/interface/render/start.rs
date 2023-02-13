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

pub fn test_quiz() -> Result<Output, Error> {
    let mut rng = rand::thread_rng();
    let rand: [f32; 6] = rng.gen();
    let program = "./target/debug/rq";
    let dir = "src/quizzes/level1/plus_minus.txt";
    let result = Command::new(program)
        .arg(dir)
        .stdout(Stdio::piped())
        .output()?;
    Ok(result)
}

fn compare_results() -> Result<String, Error> {
    let test_location = "src/quizzes/level1/plus_minus.txt";
    let answer: Vec<(f32, f32, f32)> = vec![(0.500000, 0.333333, 0.166667)];
    // load file or panic
    let path = String::from(test_location);
    let input = fs::read_to_string(path).unwrap();

    let result = test_quiz();

    let result_output;
    if result.is_ok() {
        // : Vec<(i32, i32, i32)>
        result_output = String::from_utf8(result.unwrap().stdout).unwrap();
        // assert_eq!(answer, unwrapped);
    } else {
        result_output = format!("{:?}", result.err());
    }
    Ok(result_output)
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

    let output_str: String;
    let result_output = compare_results();
    if result_output.is_ok() {
        output_str = result_output.unwrap();
    } else {
        output_str = format!("{:?}", result_output.err());
    }

    // let timestamp = time::Instant::now();
    // format!("{:?}",timestamp)
    // let ten_secs = time::Duration::from_secs(10);
    // let count_s = 0;

    // let mut result_output: Vec<Span> = vec![];

    // let result_str = format!("{:?}", result);
    // let result_vec = result_str.split("\n");
    // for r in result_vec {
    //     result_output.push(Span::raw(r.to_string()));
    // }
    // println!("{}",result_output.len());

    // Render Container
    let container = Paragraph::new(vec![
        Spans::from(vec![Span::raw(selected_quiz.example)]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "Constraints: {};",
            selected_quiz.constraints.join("; ")
        ))]),
        Spans::from(vec![Span::raw(format!(
            "Input: {};",
            selected_quiz.input.join("; ")
        ))]),
        Spans::from(vec![Span::raw(format!(
            "Output: {};",
            selected_quiz.output.join("; ")
        ))]),
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
