use crate::{interface::types::{Error, QuizJson, QuizResults, QuizList}, quizzes::{utils::get_config, types::Quiz}};
use std::fs;

pub fn read_results() -> Result<Vec<QuizResults>, Error> {
    let db_content = fs::read_to_string("./user-data/results.json")?;
    let parsed: Vec<QuizResults> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}
pub fn read_quiz_list() -> Result<Vec<QuizList>, Error> {
    let db_content = fs::read_to_string("./src/quizzes/quizzes.json")?;
    let parsed: Vec<QuizJson> = serde_json::from_str(&db_content)?;
    let mut list: Vec<QuizList> = vec![];
    for p in parsed {
        let config = get_config(Quiz::new(&p.name, &p.level));
        list.push(QuizList {
            name: p.name,
            level: p.level,
            desc: config.desc,
            example: config.example,
            constraints: config.constraints,
        });
    }

    Ok(list)
}
