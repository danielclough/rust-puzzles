pub fn test(arr: &str) -> Vec<i32> {    
    let inputs  = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for input in inputs {
        answers.push(birthday(&input.ar, input.d, input.m));
    };
    answers
}

#[derive(Clone)]
struct Input {
    ar: Vec<i32>,
    d: i32,
    m: i32,
}

// line1 == n squares of chocolate bar
// line2 == i32_arr
// line3 == d (birth day) and m (birth month)
fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![Input {ar: vec![], d: 0, m: 0}];
    let mut output_n = 0;

    for (i, line) in lines.iter().enumerate() {
        if i % 4 == 1 {
            output[output_n].ar = line.split(" ").map(|x| x.parse::<i32>().expect("number")).collect();
        }
        else if i % 4 == 2 {
            output[output_n].d = line.parse::<i32>().expect("number");
        }
        else if i % 4 == 3 {
            output[output_n].m = line.parse::<i32>().expect("number");
            output_n += 1;
        }
    }
    output
}

#[allow(non_snake_case)]
fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    println!("{} {} {:?}", d, m, s);
    5
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![ 2,0,1 ];

        // load file or panic
        let path = String::from("input/week3/subarray_division_2.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}