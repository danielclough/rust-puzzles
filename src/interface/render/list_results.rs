use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table},
};

use crate::interface::controllers::read_results;

pub fn render<'a>(result_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let results = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Attempt")
        .border_type(BorderType::Plain);

    let result_list = read_results().expect("can fetch result list");
    
    let items: Vec<_> = result_list
        .iter()
        .map(|result| {
            ListItem::new(Spans::from(vec![Span::styled(
                result.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_result = result_list
        .get(
            result_list_state
                .selected()
                .expect("there is always a selected result"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(results).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let result_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_result.name.to_string())),
        Cell::from(Span::raw(selected_result.level.to_string())),
        Cell::from(Span::raw(selected_result.path_name.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Level",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Path",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Results")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ]);

    (list, result_detail)
}
