use crate::interface::types::{Error, QuizResults, QuizList};
use std::fs;

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
