use crate::interface::types::{Error, QuizResults};
use std::fs;

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
