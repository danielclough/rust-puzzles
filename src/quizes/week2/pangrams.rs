pub fn test(arr: &str) -> Vec<String> {    
    let strings  = read_input(arr);
    let mut answers: Vec<String> = vec![];
    for string in strings {
        answers.push(pangrams(&string));
    };
    answers
}

fn read_input(arr: &str) -> Vec<String> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];
    
    for line in lines {
        output.push(line.to_string());
    }
    output
}


fn pangrams(s: &str) -> String {
    let alphabet: Vec<&str> = "abcdefghijklmnopqrstuvwxyz".split("").collect();
    let mut count_arr = vec![0;alphabet.len()];
    let str_arr: Vec<&str> = s.split("").collect();
    let mut answer: String = String::from("pangram");
    for str in str_arr {
        for (i, alpha) in alphabet.iter().map(|x| x.to_owned()).enumerate() {
            if str.to_lowercase() == alpha {
                count_arr[i] += 1;
            };
        }
    }
    for c in &count_arr {
        if c == &0 {
            answer = String::from("not pangram");
            break;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![ 
            String::from("pangram"),
            String::from("not pangram"),
           ];

        // load file or panic
        let path = String::from("input/week2/pangrams.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}