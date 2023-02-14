use crate::interface::types::{Error, QuizResults};
use std::fs;

pub fn log_user_results(results: QuizResults) -> Result<Vec<QuizResults>, Error> {
    let path = "./user-data/results.json";
    let db_content = fs::read_to_string(path)?;
    let mut parsed: Vec<QuizResults> = serde_json::from_str(&db_content)?;

    parsed.push(results);
    fs::write(path, serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}
