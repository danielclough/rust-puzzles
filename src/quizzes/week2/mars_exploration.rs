pub fn test(arr: &str) -> Vec<i32> {
    let strings = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for str in strings {
        answers.push(marsExploration(&str));
    }
    answers
}

fn read_input(arr: &str) -> Vec<&str> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for line in lines {
        output.push(line);
    }
    output
}

#[allow(non_snake_case)]
fn marsExploration(s: &str) -> i32 {
    let str_arr: Vec<&str> = s.split("").collect();
    let mut deranged = 0;
    for (i, str) in str_arr.iter().filter(|&x| !x.is_empty()).enumerate() {
        if i % 3 == 0 || i % 3 == 2 {
            if str != &"S" {
                deranged += 1;
                println!("NotS {}", str);
            }
        } else if i % 3 == 1 {
            if str != &"O" {
                deranged += 1;
                println!("NotO {}", str);
            }
        }
    }
    deranged
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let answer = vec![3, 1, 0];

        // load file or panic
        let path = String::from("input/week2/mars_exploration.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");

        assert_eq!(answer, test(&input));
    }
}
