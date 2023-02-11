use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct QuizResults {
    pub enum_name: String,
    pub week: String,
    pub path_name: String,
}
#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}
pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    List,
    Results
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::List => 1,
            MenuItem::Results => 2,
        }
    }
}

pub struct MenuConfig {
    pub titles: Vec<&'static str>,
    pub active_item: MenuItem,
}
impl MenuConfig {
    pub fn new() -> MenuConfig {
        MenuConfig {
            titles: vec!["Home", "Start", "List", "Results", "Quit"],
            active_item: MenuItem::Home,
        }
    }
}