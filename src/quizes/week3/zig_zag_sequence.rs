pub fn test(arr: &str) -> Vec<String> {    
    let inputs  = read_input(arr);
    let mut answers: Vec<String> = vec![];
    for input in inputs {
        answers.push(zig_zag_sequence(input));
    };
    answers
}

#[derive(Clone)]
struct Input {
// t == n_of_tests
    t: i32,
// n == lenght of arr_a
    n: i32,
// a == array of 132
    a: Vec<i32>,
}

fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output: Vec<Input> = vec![Input {t:0,n:0,a:vec![]};lines.len()/3];
    let mut output_i = 0;

    for (i, line) in lines.iter().enumerate() {
        if i % 3 == 0 {
            // t == n_of_tests
            output[output_i].t = line.parse::<i32>().expect("number");
        }
        else if i % 3 == 1 {
            // n == lenght of arr_a
            output[output_i].n = line.parse::<i32>().expect("number");
        }
        else if i % 3 == 2 {
            // a == array of 132
            output[output_i].a = line.split(" ").map(|x| x.parse::<i32>().expect("number")).collect();
            output_i += 1;
        }
    }
    output
}

fn zig_zag_sequence(input: Input) -> String {

    let output = String::from("");
    println!("{} {} {:?}", input.t, input.n, input.a);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![ String::from("1 2 3 7 6 5 4") ];

        // load file or panic
        let path = String::from("input/week3/zig_zag_sequence.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}