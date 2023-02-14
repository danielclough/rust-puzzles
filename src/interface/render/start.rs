use rand::Rng;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::interface::{
    controllers::{
        start_quiz::{init_compare, get_selected_quiz, get_elapsed, create_file_if_needed},
        log::log_user_results,
    },
    types::{QuizList, QuizResults},
};

pub fn log_results_and_cleanup(selected_quiz: &QuizList) {
    //     // create random number for attempt
    let mut rand_thread = rand::thread_rng();
    let rand_n: [i32;1] = rand_thread.gen();
    let id: String = rand_n.map(|x| x.to_string()).join("");
    let result = QuizResults {
        id: id.to_owned(),
        name: selected_quiz.name.to_string(),
        level: selected_quiz.level.to_string(),
        elapsed: get_elapsed(),
    };
    _ = log_user_results(result);
    // change to Results
    _ = std::fs::rename(
        "./user-data/src/main.rs",
        format!("./user-data/src/{}-{}.rs", selected_quiz.name.to_string(), id)
    );
    _ = std::fs::remove_file("./user-data/.timestamp");
}

pub fn render<'a>(quiz_list_state: &ListState) -> Paragraph<'a> {
    // init state
    let selected_quiz = get_selected_quiz(quiz_list_state);
    let comparison = init_compare(&selected_quiz);
    
    // cleanup on completion
    if comparison.is_correct && comparison.elapsed.ne("0:0:0") {
        _ = log_results_and_cleanup(&selected_quiz);
    } else {
        _ = create_file_if_needed().expect("file should be created");
    }

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
