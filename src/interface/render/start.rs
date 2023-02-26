use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::interface::{
    controllers::{
        start_quiz::{init_compare, get_selected_quiz, create_file_if_needed, log_results_and_cleanup},
    },
};

pub fn render<'a>(quiz_list_state: &ListState) -> Paragraph<'a> {
    // init state
    let selected_quiz = get_selected_quiz(quiz_list_state);
    let comparison = init_compare(&selected_quiz);
    
    // cleanup on completion
    if comparison.is_correct && comparison.elapsed.ne("0:0:0") {
        _ = log_results_and_cleanup(&selected_quiz);
    } else {
        _ = create_file_if_needed(&selected_quiz).expect("file should be created");
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
        Spans::from(comparison.user_err),
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
