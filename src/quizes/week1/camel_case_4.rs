// // Use to get stdin in HackerRank
// use std::io;
// fn main() {
//     let lines = readlines();
    // camel_case_4(lines);
// }
// fn readlines() -> Vec<String> {
//     use io::prelude::*;
//     let stdin = io::stdin();
//     let v = stdin.lock().lines().map(|x| x.unwrap()).collect();
//     v
// }

pub fn test(arr: &str) {
    let split: Vec<&str> = arr.split("\n").collect();

    let mut lines: Vec<String> = vec![];
    
    for line in split {
        lines.push(line.to_owned());
    }

    camel_case_4(lines);
}

fn camel_case_4(lines: Vec<String>) {
    for line in lines {
        let line_parts:Vec<&str> = line.split(';').collect();
        // let str_final: &str;
        
        // line_parts[2] is the words required vecs are below
        let words: Vec<&str>;
        let word_str = line_parts[2].to_owned();

        // line_parts[0] is either S or C
        // line_parts[1] is V, M, or C
        // S split
        if line_parts[0] == "S" {
            let split_str = split_on_upper(word_str.clone());
            let mut lower_str = make_lower(&split_str.clone());
            while lower_str.chars().any(|c| matches!(c, 'A'..='Z')) {
                lower_str = make_lower(&lower_str.clone());
            }

            // V variable
            if line_parts[1] == "V" {
                println!("{}", lower_str);
            } 
            // M method
            else if line_parts[1] == "M" {
                println!("{}", &lower_str.replace(&['(',')'], ""));
            }
            // C class
            if line_parts[1] == "C" {
                println!("{}", lower_str);
            } 
        }
        // C combine
        else {
            // create arr for mutating
            let mut word_arr = vec![];
            // create str for final
            let mut combined:String;
            // split space seperated words
            words = line_parts[2].split(' ').collect();
            // V variable
            if line_parts[1] == "V" {
                let mut n= 0;
                for word in words {
                    if n!=0 {
                        word_arr.push(caps_first_letter(&word));
                    } else {
                        word_arr.push(word.to_owned());
                        n+=1;
                    }
                }
                combined = word_arr.iter().cloned().collect::<String>();
                println!("{}", combined);
            } 
            // M method
            else if line_parts[1] == "M" {
                let mut n= 0;
                for word in words {
                    if n!=0 {
                        word_arr.push(caps_first_letter(&word));
                    } else {
                        word_arr.push(word.to_owned());
                        n+=1;
                    }
                }
                combined = word_arr.iter().cloned().collect::<String>();
                combined.insert(combined.len(), '(');
                combined.insert(combined.len(), ')');
                println!("{}", combined);
            }
            // C class
            else if line_parts[1] == "C" {
                for word in words {
                    word_arr.push(caps_first_letter(&word));
                }
                combined = word_arr.iter().cloned().collect::<String>();
                println!("{}", combined);
            } 
        }
    }
    
}

fn caps_first_letter(pt: &str) -> String {
    let str_formated = &format!("{}{}", &pt[0..1].to_uppercase(), &pt[1..]);
    str_formated.to_owned()
}

fn split_on_upper(mut str: String) -> String {
    // iter to keep track of correct position while mutating
    let mut times = 0;
    for (i, c) in str.to_owned().chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            str.insert(i+times, ' ');
            times += 1;
        }
    }
    str.to_owned()
}

fn make_lower(str: &str) -> String {
    let mut new = String::from("");
    for (_i,c) in str.chars().enumerate() {
        if c.is_uppercase() {
            let correct = c.to_lowercase().to_string();
            new = str.replace(c, &correct);
            break;
        }
    }
    new.to_owned()
}