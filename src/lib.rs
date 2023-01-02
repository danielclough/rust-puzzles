mod quizes;

pub fn test(quiz: Quiz) {
    quizes::match_quiz(quiz);
}

pub struct Quiz {
    pub week: String,
    pub name: String,
    pub input: String,
}

impl Quiz {
    pub fn new(path: &String, input: &String, week: &String) -> Quiz {
        let name = path.clone();
        Quiz {
            name,
            week: week.clone(),
            input: input.clone(),
        }
    }
}
