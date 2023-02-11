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

use crate::interface::controllers::read_db;
use crate::interface::render::{
    menu::{menu_component, tabs_component},
    copyright::{copyright_component},
    home,
};
use crate::interface::types::{Event, MenuConfig, MenuItem};

use super::render::{list_results, list_quizzes};


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
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = copyright_component();
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
                    let (left, right) = list_quizzes::render(&quiz_list_state);
                    rect.render_stateful_widget(left, quizzes_chunks[0], &mut quiz_list_state);
                    rect.render_widget(right, quizzes_chunks[1]);
                },
                MenuItem::Results => {
                    let quizzes_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = list_results::render(&quiz_result_state);
                    rect.render_stateful_widget(left, quizzes_chunks[0], &mut quiz_result_state);
                    rect.render_widget(right, quizzes_chunks[1]);
                }
            }
            rect.render_widget(copyright, chunks[2]);
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
                KeyCode::Char('s') => {
                    {};
                }
                KeyCode::Char('r') => {
                    {};
                }
                KeyCode::Down => {
                    if let Some(selected) = quiz_list_state.selected() {
                        let amount_quizzes = read_db("./input/quizzes.json").expect("can fetch quiz list").len();
                        if selected >= amount_quizzes - 1 {
                            quiz_list_state.select(Some(0));
                        } else {
                            quiz_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = quiz_list_state.selected() {
                        let amount_quizzes = read_db("./input/quizzes.json").expect("can fetch quiz list").len();
                        if selected > 0 {
                            quiz_list_state.select(Some(selected - 1));
                        } else {
                            quiz_list_state.select(Some(amount_quizzes - 1));
                        }
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
