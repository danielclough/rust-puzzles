use std::time::{SystemTime, UNIX_EPOCH};

// use rand::Rng;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::interface::{
    controllers::{read::read_quiz_list, start_quiz::compare_results},
    types::{Comparison, QuizList},
};

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
    //     // create random number for attempt
    // let mut rand_thread = rand::thread_rng();
    // let rand_n: [i32;6] = rand_thread.gen();

    let elapsed = get_elapsed();

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
    let correct_str = correct_vec.join("; ");
    let user_str = user_vec.join("; ");

    let comparison = Comparison {
        input_str: input_str.to_string(),
        correct_str,
        user_str,
        is_correct: comparison_tupl.3,
        elapsed,
    };
    comparison
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

    let comparison = init_compare(&selected_quiz);

    // Render Container
    let container = Paragraph::new(vec![
        Spans::from(vec![Span::raw(selected_quiz.example)]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "Constraints: {};",
            selected_quiz.constraints.join("; ")
        ))]),
        Spans::from(vec![Span::raw(format!("Input: {};", comparison.input_str))]),
        Spans::from(vec![Span::raw(format!(
            "Output: {};",
            comparison.correct_str
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("  User Output:")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(comparison.user_str),
        // Spans::from(vec![Span::raw(format!("{:?}",user_vec))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "Correct? {};",
            comparison.is_correct
        ))]),
    ])
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(format!("Quiz in progress: {} ({:?})", selected_quiz.name, comparison.elapsed))
            .border_type(BorderType::Plain),
    );
    container
}
