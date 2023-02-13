use crate::interface::{
    controllers::read,
    types::{MenuConfig, MenuItem},
};
use tui::widgets::ListState;

pub fn exec(menu_config: &MenuConfig, quiz_list_state: &mut ListState) {
    match menu_config.active_item {
        MenuItem::List => {
            if let Some(selected) = quiz_list_state.selected() {
                let n_items = read::read_quiz_list().expect("can fetch quiz list").len();
                if selected > 0 {
                    quiz_list_state.select(Some(selected - 1));
                } else {
                    quiz_list_state.select(Some(n_items - 1));
                }
            }
        }
        MenuItem::Results => {
            if let Some(selected) = quiz_list_state.selected() {
                let n_items = read::read_results().expect("can fetch quiz list").len();
                if selected > 0 {
                    quiz_list_state.select(Some(selected - 1));
                } else {
                    quiz_list_state.select(Some(n_items - 1));
                }
            }
        }
        _ => {}
    }
}
