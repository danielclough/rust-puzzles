use std::{
    io::Stdout,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Terminal,
};
use crate::interface::render::{
    home,
    list_results, list_quizzes,
    menu::{menu_component, tabs_component},
};
use crate::interface::types::{MenuConfig, MenuItem};

pub fn exec(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    menu_config: &MenuConfig,
    mut quiz_list_state: &mut ListState,
    mut quiz_result_state: &mut ListState,

) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                    ]
                    .as_ref(),
                )
                .split(size);

            let menu = menu_component(menu_config.titles.to_owned());
            let tabs = tabs_component(menu, menu_config.active_item);

            rect.render_widget(tabs, chunks[0]);
            
            match menu_config.active_item {
                MenuItem::Home => rect.render_widget(home::render(), chunks[1]),
                MenuItem::List => {
                    let quizzes_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);

                    let quizzes_chunks_right = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Percentage(20),
                                Constraint::Percentage(40),
                                Constraint::Percentage(20),
                                Constraint::Percentage(20),
                                ].as_ref(),
                        )
                        .split(quizzes_chunks[1]);
                        
                    let (left, quiz_outline, quiz_desc, quiz_constraints, quiz_in_out) = list_quizzes::render(&quiz_list_state);
                    rect.render_stateful_widget(left, quizzes_chunks[0], &mut quiz_list_state);
                    rect.render_widget(quiz_outline, quizzes_chunks_right[0]);
                    rect.render_widget(quiz_desc, quizzes_chunks_right[1]);
                    rect.render_widget(quiz_constraints, quizzes_chunks_right[2]);
                    rect.render_widget(quiz_in_out, quizzes_chunks_right[3]);
                },
                MenuItem::Results => {
                    let results_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = list_results::render(&quiz_result_state);
                    rect.render_stateful_widget(left, results_chunks[0], &mut quiz_result_state);
                    rect.render_widget(right, results_chunks[1]);
                }
            }
        })?;
        Ok(())
}