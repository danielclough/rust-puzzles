use crate::interface::types::{Error, QuizResults};
// use chrono::Utc;
use std::fs;
use tui::widgets::ListState;

use super::types::QuizList;

pub fn read_results() -> Result<Vec<QuizResults>, Error> {

    let db_content = fs::read_to_string("./user-data/results.json")?;
    let parsed: Vec<QuizResults> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}
pub fn read_quiz_list() -> Result<Vec<QuizList>, Error> {

    let db_content = fs::read_to_string("./src/quizzes/input/quizzes.json")?;
    let parsed: Vec<QuizList> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn log_quiz_list(path: &str) -> Result<Vec<QuizList>, Error> {
    let db_content = fs::read_to_string(path)?;
    let mut parsed: Vec<QuizList> = serde_json::from_str(&db_content)?;

    let random_quiz = QuizList {
        name: String::new(),
        level: String::new(),
        desc: String::new(),
        example: String::new(),
        constraints: vec![String::new()],
        input: vec![String::new()],
        output: vec![String::new()],
    };

    parsed.push(random_quiz);
    fs::write(path, serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn log_user_results(path: &str) -> Result<Vec<QuizResults>, Error> {
    let db_content = fs::read_to_string(path)?;
    let mut parsed: Vec<QuizResults> = serde_json::from_str(&db_content)?;

    let random_quiz = QuizResults {
        name: "".to_string(),
        level: "".to_string(),
        path_name: "".to_string(),
    };

    parsed.push(random_quiz);
    fs::write(path, serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn remove_quiz_at_index(quiz_list_state: &mut ListState, path: &str) -> Result<(), Error> {
    if let Some(selected) = quiz_list_state.selected() {
        let db_content = fs::read_to_string(path)?;
        let mut parsed: Vec<QuizResults> = serde_json::from_str(&db_content)?;
        parsed.remove(selected);
        fs::write(path, serde_json::to_vec(&parsed)?)?;
        // let amount_quizzes = read_db().expect("can fetch quiz list").len();
        if selected > 0 {
            quiz_list_state.select(Some(selected - 1));
        } else {
            quiz_list_state.select(Some(0));
        }
    }

    Ok(())
}
