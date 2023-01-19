pub fn test(arr: &str) -> Vec<i32> {    
    let inputs  = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for input in inputs {
        answers.push(pageCount(input.n, input.p));
    };
    answers
}
#[derive(Clone)]
struct Input  {
    //  number of pages in the book
    n: i32,
    //  page to turn to
    p: i32,
}

fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output: Vec<Input> = vec![Input {n:0,p:0}; lines.len()/2];
    let mut output_i = 0;
    for (i, line) in lines.iter().enumerate() {
        let int32 = line.parse::<i32>().expect("number");
        if i%2 == 0 {
            output[output_i].n = int32;
        }
        else if i%2 == 1 {
            output[output_i].p = int32;
            output_i += 1;
        }
    }
    output
}

// book is a zero indexed array
// Return min number of pages to flip (one at a time) to page p
// book is n pages long
// start at beggining or end
// counting starts at 1 or n
#[allow(non_snake_case)]
fn pageCount(n: i32, p: i32) -> i32 {
    println!("{} {}", n, p);
    3
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![ 1,0 ];

        // load file or panic
        let path = String::from("input/week3/drawing_book.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}