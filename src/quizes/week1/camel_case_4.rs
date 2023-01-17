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

pub fn test(arr: &str) -> Vec<String> {
    let split: Vec<&str> = arr.split("\n").collect();
    let mut answers: Vec<String> = vec![];
    
    for line in split {
        answers.push(camel_case_4(line.to_owned()));
    }

    answers
}

fn camel_case_4(line: String) -> String {
    let mut answer: String = String::from("");
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
                answer = lower_str;
            } 
            // M method
            else if line_parts[1] == "M" {
                let method_str = &lower_str.replace(&['(',')'], "");
                answer = method_str.to_owned();
            }
            // C class
            else if line_parts[1] == "C" {
                answer = lower_str;
            } 
        }
        // C combine
        else if line_parts[0] == "C" {
            // create arr for mutating
            let mut word_arr = vec![];
            // create str for final
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
                answer = word_arr.iter().cloned().collect::<String>();
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
                answer = word_arr.iter().cloned().collect::<String>();
                answer.insert(answer.len(), '(');
                answer.insert(answer.len(), ')');
            }
            // C class
            else if line_parts[1] == "C" {
                for word in words {
                    word_arr.push(caps_first_letter(&word));
                }
                answer = word_arr.iter().cloned().collect::<String>();
            } 
        }
    println!("{}", answer);
    answer
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let test_location = "input/week1/camel_case_4.txt";
        let answer: Vec<String> = vec![
            String::from("plastic cup"),
            String::from("mobilePhone"),
            String::from("CoffeeMachine"),
            String::from("large software book"),
            String::from("whiteSheetOfPaper()"),
            String::from("picture frame"), 
        ];
        // load file or panic
        let path = String::from(test_location);
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        let my_answer = test(&input);
        assert_eq!(answer, my_answer);
    }
}