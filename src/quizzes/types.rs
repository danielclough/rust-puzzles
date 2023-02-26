pub struct Quiz {
    pub name: String,
    pub level: String,
    pub input: String,
    pub path: String,
}

impl Quiz {
    pub fn new(path: &String, input: &String, level: &String) -> Quiz {
        Quiz {
            name: path.clone(),
            level: level.clone(),
            input: input.clone(),
            path: path.clone(),
        }
    }
}


pub struct QuizConfig {
    pub name: String,
    pub level: String,
}

pub enum OutputType {
    VecVecI32,
    VecString,
    VecI32,
    VecTupleF32F32F32,
    I32,
    VecVecI64,
}
