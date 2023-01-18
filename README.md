# rust-puzzles

My implementation of Hacker Rank puzzles in rust.

## Motivation
After reading the first half of [The Rust Programing Language (w/ quizzes by brown.edu)](https://rust-book.cs.brown.edu/) and doing about half of [Rustlings](https://github.com/rust-lang/rustlings/) I don't know all the language features, but I know enough to practice the bare bone basics.

The problem set I use comes from the [HackerRank](https://https://www.hackerrank.com/challenges) Three Month Interview Prep course. I have found these challenges to relate to problems that I have faced in my own projects.

The only copy/pastes from HackerRank are each problem input and the function declaration required to paste my function back into their system to ensure a correct output.
I don't think this is in violation of copyright.

> **Do not copy my code while doing HackerRank challenges!**
> 
> The purpose of Hacker Rank is to showcase **your** skills.
> They have a copy of all of my code and **will catch you if you are cheating.**

The end goal of this project is for me to have a time trial for practicing common challenges using my preffered toolchain.

## Use 

### Run Tests
`cargo test`

### Run indivdual test files
#### CLI path
`cargo run input/week1/plus_minus.txt`

#### .env path
Or, add path variables to `.env` and use `cargo run`
```
CURRENT_WEEK="week1"
CURRENT_FILE="plus_minus.txt"
```
