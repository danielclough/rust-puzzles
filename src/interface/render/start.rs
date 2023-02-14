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
    let correct_str = correct_vec.join("; ");
    let user_str = user_vec.join("; ");

    let comparison = Comparison {
        input_str: input_str.to_string(),
        correct_str,
        user_str,
        is_correct: comparison_tupl.3,
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
            .title(format!("Quiz in progress: {} ", selected_quiz.name))
            .border_type(BorderType::Plain),
    );
    container
}
