pub struct Quiz {
    pub name: String,
    pub level: String,
}

impl Quiz {
    pub fn new(name: &String, level: &String) -> Quiz {
        Quiz {
            name: name.clone(),
            level: level.clone(),
        }
    }
}


pub struct QuizConfig {
    pub name: String,
    pub desc: String,
    pub example: String,
    pub constraints: Vec<String>,
    pub level: String,
    pub answer: AnswerType,
}
#[derive(PartialEq, Debug)]
pub enum AnswerType {
    VecVecI32 {answer: Vec<Vec<i32>>},
    VecString {answer: Vec<String>},
    VecI32 {answer: Vec<i32>},
    VecTupleF32F32F32 {answer: Vec<(f32,f32,f32)>},
    I32 {answer: i32},
    VecVecI64 {answer: Vec<Vec<i64>>},
}

impl AnswerType {
    
}
pub enum OutputType {
    VecVecI32,
    VecString,
    VecI32,
    VecTupleF32F32F32,
    I32,
    VecVecI64,
}
