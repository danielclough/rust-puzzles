# Rust Quizzes

My implementation of Hacker Rank challanges in rust.

## Motivation
After reading the first half of [The Rust Programing Language (w/ quizzes by brown.edu)](https://rust-book.cs.brown.edu/) and doing about half of [Rustlings](https://github.com/rust-lang/rustlings/) I was ready to play a bit.

The end goal of this project is for me to have a time trial for practicing common challenges using my preffered toolchain.

## Testing
[![Code Coverage](https://codecov.io/gh/danielclough/rust-quizzes/branch/main/graph/badge.svg?token=ZSYEUFAGUV)](https://codecov.io/gh/danielclough/rust-quizzes)
![Code Coverage](https://codecov.io/gh/danielclough/rust-quizzes/branch/main/graphs/sunburst.svg?token=ZSYEUFAGUV)

## Technical Design Document
[Read the TDD]()

## Use 

### Run Tests
`cargo test`

### Run indivdual test files
#### CLI path
`cargo run input/level1/plus_minus.txt`

#### .env path
Or, add path variables to `.env` and use `cargo run`
```
CURRENT_WEEK="level1"
CURRENT_FILE="plus_minus.txt"
```
