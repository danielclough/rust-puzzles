// use rand::Rng;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Wrap},
};

use crate::interface::{
    controllers::{
        start_quiz::{init_compare, get_selected_quiz},
    },
};

pub fn render<'a>(quiz_list_state: &ListState) -> Paragraph<'a> {
    // init state
    let selected_quiz = get_selected_quiz(quiz_list_state);
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
