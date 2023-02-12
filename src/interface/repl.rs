use std::{
    io::{self},
    sync::mpsc::Receiver,
};

use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::disable_raw_mode,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Terminal,
};

use crate::interface::controllers::read_quiz_list;
use crate::interface::render::{
    menu::{menu_component, tabs_component},
    home,
};
use crate::interface::types::{Event, MenuConfig, MenuItem};

use super::{render::{list_results, list_quizzes}, controllers::read_results};


pub fn interface(rx: Receiver<Event<KeyEvent>>) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut menu_config = MenuConfig::new();

    let mut quiz_list_state = ListState::default();
    quiz_list_state.select(Some(0));

    let mut quiz_result_state = ListState::default();
    quiz_result_state.select(Some(0));

    loop {
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
                                Constraint::Percentage(25),
                                Constraint::Percentage(50),
                                Constraint::Percentage(25)
                                ].as_ref(),
                        )
                        .split(quizzes_chunks[1]);
                        
                    let (left, quiz_outline, quiz_desc, quiz_in_out) = list_quizzes::render(&quiz_list_state);
                    rect.render_stateful_widget(left, quizzes_chunks[0], &mut quiz_list_state);
                    rect.render_widget(quiz_outline, quizzes_chunks_right[0]);
                    rect.render_widget(quiz_desc, quizzes_chunks_right[1]);
                    rect.render_widget(quiz_in_out, quizzes_chunks_right[2]);
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

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                KeyCode::Char('h') => menu_config.active_item = MenuItem::Home,
                KeyCode::Char('l') => menu_config.active_item = MenuItem::List,
                KeyCode::Char('r') => menu_config.active_item = MenuItem::Results,
                KeyCode::Char('s') => {
                    {};
                }
                KeyCode::Down => {
                    match menu_config.active_item {
                        MenuItem::Home => {},
                        MenuItem::List => {
                            if let Some(selected) = quiz_list_state.selected() {
                                let n_items = read_quiz_list().expect("can fetch quiz list").len();
                                if selected >= n_items - 1 {
                                    quiz_list_state.select(Some(0));
                                } else {
                                    quiz_list_state.select(Some(selected + 1));
                                }
                            }                            
                        },
                        MenuItem::Results => {
                            if let Some(selected) = quiz_list_state.selected() {
                                let n_items = read_results().expect("can fetch results").len();
                                if selected >= n_items - 1 {
                                    quiz_list_state.select(Some(0));
                                } else {
                                    quiz_list_state.select(Some(selected + 1));
                                }
                            }                            
                        },
                    }
                }
                KeyCode::Up => {
                    match menu_config.active_item {
                        MenuItem::Home => {},
                        MenuItem::List => {
                            if let Some(selected) = quiz_list_state.selected() {
                                let n_items = read_quiz_list().expect("can fetch quiz list").len();
                                if selected > 0 {
                                    quiz_list_state.select(Some(selected - 1));
                                } else {
                                    quiz_list_state.select(Some(n_items - 1));
                                }
                            }                            
                        },
                        MenuItem::Results => {
                            if let Some(selected) = quiz_list_state.selected() {
                                let n_items = read_results().expect("can fetch quiz list").len();
                                if selected > 0 {
                                    quiz_list_state.select(Some(selected - 1));
                                } else {
                                    quiz_list_state.select(Some(n_items - 1));
                                }
                            }                            
                        },
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

