use std::{env,fs,path::Path,ffi::OsStr};
use dotenv::dotenv;

use puzzles::{test, Quiz};

fn main() {
    let input;
    let week: String;
    let path: String;
    let name: String;
    
    // check args for current path
    // check .env if not found
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        dotenv().expect("Failed to load .env file");
        let filename = env::var("CURRENT_FILE").unwrap();
        let filename_parts: Vec<&str> = filename.split('.').collect();
        name = filename_parts[0].to_owned().clone();
        week = env::var("CURRENT_WEEK").unwrap();
        path = format!("input/{week}/{name}.txt");
    } else {
        path = args[1].to_owned();
        let path_parts: Vec<&str> = path.split('/').collect();
        week = path_parts[1].to_owned();
        let filename = filename(&path).to_str().expect("to_str");
        let filename_parts: Vec<&str> = filename.split('.').collect();
        name = filename_parts[0].to_owned().clone();
    }
    println!("{}", path);

    // load file or panic
    input = fs::read_to_string(&path).expect("Should have been able to read the file");
   

    let quiz = Quiz::new(&name, &input, &week);
    test(quiz);
}

fn filename(file: &str) -> &OsStr {
    let path_from_dotenv = Path::new(file);
    let filename = path_from_dotenv.file_name().expect("works");
    filename
}