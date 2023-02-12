use std::{
    io,
    sync::mpsc::Receiver,
};

use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::disable_raw_mode,
};
use tui::{
    backend::CrosstermBackend,
    widgets::ListState,
    Terminal,
};

use crate::interface::controllers::{key_up, key_down, start_quiz};
use crate::interface::types::{Event, MenuConfig, MenuItem};

use super::render::draw_terminal;

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
        _ = draw_terminal::exec(&mut terminal, &menu_config, &mut quiz_list_state, &mut quiz_result_state);

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
                KeyCode::Char('s') => start_quiz::exec(),
                KeyCode::Down => key_down::exec(&menu_config, &mut quiz_list_state),
                KeyCode::Up => key_up::exec(&menu_config, &mut quiz_list_state),
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
