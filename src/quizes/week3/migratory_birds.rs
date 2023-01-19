pub fn test(arr: &str) -> Vec<i32> {    
    let inputs  = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for input in inputs {
        answers.push(migratoryBirds(&input));
    };
    answers
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        if i % 2 == 1 {
            output.push(line.split(" ").map(|x| x.parse::<i32>().expect("number")).collect());
        }
    }
    output
}

#[allow(non_snake_case)]
fn migratoryBirds(arr: &[i32]) -> i32 {
    println!("{:?}", arr);
    7
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![ 4, 3 ];

        // load file or panic
        let path = String::from("input/week3/migratory_birds.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}