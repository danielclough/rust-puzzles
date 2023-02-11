use crate::interface::types::{Error, QuizResults};
// use chrono::Utc;
use rand::{Rng};
use std::fs;
use tui::widgets::ListState;

pub fn read_db(path: &str) -> Result<Vec<QuizResults>, Error> {

    let db_content = fs::read_to_string(path)?;
    let parsed: Vec<QuizResults> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn log_user_results(path: &str) -> Result<Vec<QuizResults>, Error> {
    let mut rng = rand::thread_rng();
    let db_content = fs::read_to_string(path)?;
    let mut parsed: Vec<QuizResults> = serde_json::from_str(&db_content)?;
    let quiz_types = match rng.gen_range(0..1) {
        0 => "Indian",
        _ => "Chinese",
    };

    let random_quiz = QuizResults {
        enum_name: "".to_string(),
        week: "".to_string(),
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
