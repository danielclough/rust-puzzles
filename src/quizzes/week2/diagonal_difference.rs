pub fn quiz(arr: &str) -> i32 {
    let ints = read_input(arr);
    diagonalDifference(&ints)
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        let str_arr: Vec<&str> = line.split(' ').collect();
        let mut i32_arr: Vec<i32> = vec![];
        for ele in str_arr {
            i32_arr.push(ele.parse::<i32>().expect("number here"));
        }

        if i != 0 {
            output.push(i32_arr);
        }
    }
    output
}

#[allow(non_snake_case)]
fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    // println!("{:?}", arr);
    let mut primary_diag = 0;
    let mut secondary_diag = 0;
    for (i, a) in arr.iter().enumerate() {
        primary_diag = primary_diag + a[i];
        secondary_diag = secondary_diag + a[a.len() - i - 1];
    }
    let output = secondary_diag - primary_diag;
    output.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let answer = 15;

        // load file or panic
        let path = "input/week2/diagonal_difference.txt";
        let input = fs::read_to_string(path).unwrap();

        assert_eq!(answer, quiz(&input));
    }
}
