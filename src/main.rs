use std::{env,fs,path::Path,ffi::OsStr};
use dotenv::dotenv;

use puzzles::{test, Quiz};

fn main() {
    // check .env and args for current path
    dotenv().expect("Failed to load .env file");
    let name_from_dotenv = env::var("CURRENT_FILE").unwrap();
    let path_from_dotenv = "input/week1/".to_owned() + &name_from_dotenv;

    let args: Vec<String> = env::args().collect();

    let input;
    let path: &str;
    if args.len() < 2 {
        path = &path_from_dotenv;
    } else {
        path = &args[1];
    }
    println!("{}", path);
    input = fs::read_to_string(&path).expect("Should have been able to read the file");

    let filename = filename(&path).to_str().expect("to_str");
    let parts: Vec<&str> = filename.split('.').collect();
    let name = parts[0].to_owned().clone();
   

    let quiz = Quiz::new(&name, &input);
    test(quiz);
}

fn filename(file: &str) -> &OsStr {
    let path_from_dotenv = Path::new(file);
    let filename = path_from_dotenv.file_name().expect("works");
    filename
}