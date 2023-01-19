pub fn test(arr: &str) -> Vec<String> {    
    let inputs  = read_input(arr);
    let mut answers: Vec<String> = vec![];
    for input in inputs {
        answers.push(twoArrays(input.k, &input.a, &input.b));
    };
    answers
}

#[derive(Clone)]
struct Input {
    k: i32,
    a: Vec<i32>,
    b: Vec<i32>,
}

fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![Input {k: 0, a: vec![], b: vec![]}];
    let mut output_n = 0;
    let mut n = 0;

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            n = line.parse::<i32>().expect("number");
        }
        else if i-1 % 3 == 0 {
            // n space k
            let n_k: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().expect("number")).collect();
            output[output_n].k = n_k[1];
        }
        else if i-1 % 3 == 1 {
            output[output_n].a = line.split(" ").map(|x| x.parse::<i32>().expect("number")).collect();
        }
        else if i-1 % 3 == 2 {
            output[output_n].b = line.split(" ").map(|x| x.parse::<i32>().expect("number")).collect();
            if n == 0 {
                output.push(Input {k: 0, a: vec![], b: vec![]});
                output_n += 1;
                n -= 1;
            }
        }
    }
    output
}

#[allow(non_snake_case)]
fn twoArrays(k: i32, A: &[i32], B: &[i32]) -> String {
    println!("{} {:?} {:?}", k, A, B);
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![
            "YES".to_owned(),
            "NO".to_owned(),
            "NO".to_owned(),
        ];

        // load file or panic
        let path = String::from("input/week3/permuting_two_arrays.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}