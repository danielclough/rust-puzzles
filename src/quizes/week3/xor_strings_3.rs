pub fn test(arr: &str) -> Vec<&str> {    
    let inputs  = read_input(arr);
    let mut answers: Vec<&str> = vec![];
    for input in inputs {
        answers.push(xorinputs3(input));
    };
    answers
}

fn read_input(arr: &str) -> Vec<Vec<&str>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![vec![""]; lines.len()/2];
    let mut output_n = 0;

    for (i, line) in lines.iter().enumerate() {
        output[output_n].push(line);
        if i % 2 == 1 {
            output_n += 1;
        }
    }
    output
}

fn xorinputs3(input: Vec<&str>) -> &str {
    println!("{:?}", input);
    ""
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![ 
            "10000",
        ];

        // load file or panic
        let path = String::from("input/week3/xor_inputs_3.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}