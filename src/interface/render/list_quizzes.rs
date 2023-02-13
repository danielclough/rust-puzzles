use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Wrap,
    },
};

use crate::interface::{controllers::read::read_quiz_list, types::QuizList};

pub fn render<'a>(
    quiz_list_state: &ListState,
) -> (List<'a>, Table<'a>, Paragraph<'a>, Table<'a>, Table<'a>) {
    let yellow = Color::Rgb(168, 139, 24);
    let redish = Color::Rgb(117, 52, 113);
    // let bright_yellow = Color::Rgb(224,204,16);
    // let purple = Color::Rgb(113,9,219);
    // let pink = Color::Rgb(168,24,158);

    let quizzes = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::DarkGray))
        .title("Quizzes")
        .border_type(BorderType::Plain);

    let quiz_list: Vec<QuizList> = read_quiz_list().expect("can fetch quiz list");
    let items: Vec<_> = quiz_list
        .iter()
        .map(|quiz| {
            ListItem::new(Spans::from(vec![Span::styled(
                quiz.name.clone(),
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
            .bg(yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    // Outline
    let outline = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_quiz.name)),
        Cell::from(Span::raw(selected_quiz.level)),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD).fg(yellow),
        )),
        Cell::from(Span::styled(
            "Level",
            Style::default().add_modifier(Modifier::BOLD).fg(yellow),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title("Outline")
            .border_type(BorderType::Plain),
    )
    .widths(&[Constraint::Percentage(70), Constraint::Percentage(30)]);

    // Description
    // Example
    let quiz_desc = Paragraph::new(vec![
        Spans::from(vec![Span::raw(selected_quiz.desc)]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(selected_quiz.example)]),
    ])
    .wrap(Wrap { trim: false })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title("Description")
            .border_type(BorderType::Plain),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title("How to turn Input to Output")
            .border_type(BorderType::Plain),
    );

    // Constraints
    let quiz_constraints = Table::new(vec![Row::new(vec![Cell::from(Span::raw(
        selected_quiz.constraints.join(", "),
    ))])])
    .header(Row::new(vec![Cell::from(Span::styled(
        "Constraints",
        Style::default().add_modifier(Modifier::BOLD).fg(redish),
    ))]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title("Constraints")
            .border_type(BorderType::Plain),
    )
    .widths(&[Constraint::Percentage(100)]);

    // Input and Output
    let quiz_in_out = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_quiz.input.join(", "))),
        Cell::from(Span::raw(selected_quiz.output.join(", "))),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Input",
            Style::default().add_modifier(Modifier::BOLD).fg(redish),
        )),
        Cell::from(Span::styled(
            "Output",
            Style::default().add_modifier(Modifier::BOLD).fg(redish),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title("Input")
            .border_type(BorderType::Plain),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title("Output")
            .border_type(BorderType::Plain),
    )
    .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

    (list, outline, quiz_desc, quiz_constraints, quiz_in_out)
}
