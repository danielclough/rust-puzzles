// use dotenv::dotenv;
// use std::{env, ffi::OsStr, fs, path::Path};

// use rq::{interface::{types::{Event, QuizResults}, repl}, quizzes::utils::{self, Quiz}};

use crossterm::{
    event::{self, Event as CEvent},
    terminal::enable_raw_mode,
};
use rq::interface::{types::Event, repl};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // // check args for current path
    // // check .env if not found
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     dotenv().expect("Failed to load .env file");
    //     let filename = env::var("CURRENT_FILE").unwrap();
    //     let filename_parts: Vec<&str> = filename.split('.').collect();
    //     let quiz = Quiz {
    //         name: filename_parts[0].to_owned(),
    //         level: env::var("CURRENT_WEEK").unwrap(),
    //         path: format!("input/{level}/{name}.txt"),
    //     }
    // } else {
    //     path = args[1].to_owned();
    //     let path_parts: Vec<&str> = path.split('/').collect();
    //     level = path_parts[1].to_owned();
    //     let filename = filename(&path).to_str().expect("to_str");
    //     let filename_parts: Vec<&str> = filename.split('.').collect();
    //     name = filename_parts[0].to_owned();
    // }

    // // load file or panic
    // let input = fs::read_to_string(&path).expect("Should have been able to read the file");

    // let quiz = Quiz::new(&name, &input, &level);
    // utils::test(quiz);




    // TUI
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(300);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    let output = repl::interface(rx);

    output
    
}