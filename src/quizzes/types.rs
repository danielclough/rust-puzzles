pub struct Quiz {
    pub name: QuizOption,
    pub level: String,
    pub input: String,
    pub path: String,
}

impl Quiz {
    pub fn new(path: &String, input: &String, level: &String) -> Quiz {
        Quiz {
            name: QuizOption::PlusMinus,
            level: level.clone(),
            input: input.clone(),
            path: path.clone(),
        }
    }
}

pub enum QuizOption {
    PlusMinus,
    MiniMaxSum,
    TimeConversion,
    BreakingTheRecords,
    CamelCase4,
    DivisibleSumPairs,
    SparseArrays,
    LonelyInteger,
    GradingStudents,
    FlippingBits,
    DiagonalDifference,
    CountingSort1,
    CountingValleys,
    Pangrams,
    MarsExploration,
    PermutingTwoArrays,
    SubarrayDivision2,
    XorStrings3,
    SalesByMatch,
    MigratoryBirds,
    MaximumPerimeterTriangle,
    ZigZagSequence,
    DrawingBook,
}

pub struct QuizOutput {
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
