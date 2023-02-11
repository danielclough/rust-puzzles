use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table},
};

use crate::interface::controllers::read_db;

pub fn render<'a>(quiz_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let quizzes = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Quizzes")
        .border_type(BorderType::Plain);

    let quiz_list = read_db("./input/quiz.json").expect("can fetch quiz list");
    let items: Vec<_> = quiz_list
        .iter()
        .map(|quiz| {
            ListItem::new(Spans::from(vec![Span::styled(
                quiz.enum_name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_quiz = quiz_list
        .get(
            quiz_list_state
                .selected()
                .expect("there is always a selected quiz"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(quizzes).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let quiz_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_quiz.enum_name)),
        Cell::from(Span::raw(selected_quiz.week)),
        Cell::from(Span::raw(selected_quiz.path_name)),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Week",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Path Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(30),
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
    ]);

    (list, quiz_detail)
}
