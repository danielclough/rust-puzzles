pub fn test(arr: &str) -> Vec<Vec<i32>> {    
    let ints  = read_input(arr);
    let mut answers: Vec<Vec<i32>> = vec![];
    for arr in ints {
        answers.push(countingSort(&arr));
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

#[allow(non_snake_case)]
fn countingSort(arr: &[i32]) -> Vec<i32> {
    let mut output = vec![0; 100];
    for a in arr {
        output[*a as usize] += 1
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer: Vec<Vec<i32>> = vec![
            "0 2 0 2 0 0 1 0 1 2 1 0 1 1 0 0 2 0 1 0 1 2 1 1 1 3 0 2 0 0 2 0 3 3 1 0 0 0 0 2 2 1 1 1 2 0 2 0 1 0 1 0 0 1 0 0 2 1 0 1 1 1 0 1 0 1 0 2 1 3 2 0 0 2 1 2 1 0 2 2 1 2 1 2 1 1 2 2 0 3 2 1 1 0 1 1 1 0 2 2"
                .split(" ").map(|x| x.to_owned().parse::<i32>().unwrap()).collect(),
            "2 0 1 0 0 1 1 1 1 0 0 1 3 2 2 0 4 4 1 1 0 0 0 0 3 0 0 1 0 1 2 0 1 2 2 3 0 2 0 0 1 0 1 1 0 0 1 1 0 2 0 0 1 1 1 0 1 0 1 1 2 3 0 1 2 0 1 2 1 1 4 1 0 1 1 3 0 0 2 1 2 3 2 2 2 0 0 1 0 0 0 0 0 0 2 0 1 3 1 0"
                .split(" ").map(|x| x.to_owned().parse::<i32>().unwrap()).collect(),
        ];

        // load file or panic
        let path = String::from("input/week2/counting_sort_1.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}