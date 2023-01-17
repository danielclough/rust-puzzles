pub fn test(arr: &str) -> Vec<i32> {    
    let ints  = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for arr in ints {
        answers.push(pangrams(&arr));
    };
    answers
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        let str_arr: Vec<&str> = line.split(' ').collect();
        let mut i32_arr: Vec<i32> = vec![];
        for ele in str_arr {
            i32_arr.push(ele.parse::<i32>().expect("number here"));
        };

        if i%2 == 1 {
            output.push(i32_arr);
        }
    }
    output
}


fn pangrams(a: &[i32]) -> i32 {
    println!("{:?}", a);
    let output = 1;
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![ 1,2,2 ];

        // load file or panic
        let path = String::from("input/week2/pangrams.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}