pub fn test(arr: &str) -> Vec<Vec<i64>> {
    let int_arr_arr = read_input(arr);
    let mut answers_staging: Vec<i64> = vec![];
    let mut answers: Vec<Vec<i64>> = vec![];
    for int_arr in int_arr_arr {
        for int64 in int_arr {
            answers_staging.push(flippingBits(int64));
        }
        answers.push(answers_staging.to_owned());
        answers_staging = vec![];
    }
    answers
}

// line0 contains n of queries.
// following lines contain integers to process.
fn read_input(arr: &str) -> Vec<Vec<i64>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    // var to collect output
    let mut output: Vec<Vec<i64>> = vec![];
    // var to collect sets of queries
    let mut i64_arr: Vec<i64> = vec![];
    // var to count how many queries left
    let mut n_left = 0;

    // iterate through lines and
    for (i, line) in lines.iter().enumerate() {
        let parsed = line.parse::<i64>().expect("number");
        if n_left == 0 {
            if i != 0 {
                output.push(i64_arr);
            };
            n_left = parsed;
            i64_arr = vec![];
        } else {
            i64_arr.push(parsed);
            n_left -= 1;
        }
    }
    // push last case
    output.push(i64_arr);
    output
}

#[allow(non_snake_case)]
fn flippingBits(n: i64) -> i64 {
    let formated_as_u32 = format!("{:032b}", n as u32);
    let mut flipped: String = String::from("");

    for (_, s) in formated_as_u32.chars().enumerate() {
        if s == '0' {
            flipped = format!("{}{}", flipped, "1");
        } else {
            flipped = format!("{}{}", flipped, "0");
        }
    }

    // println!("{}", );

    let output: i64 = i64::from_str_radix(&flipped, 2).unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let answer: Vec<Vec<i64>> = vec![
            vec![2147483648, 4294967294, 4294967295],
            vec![4294967291, 4294843839],
            vec![4294967295, 3492223820, 4259365872],
        ];

        // load file or panic
        let path = String::from("input/week2/flipping_bits.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");

        assert_eq!(answer, test(&input));
    }
}
