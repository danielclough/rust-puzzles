pub fn test(arr: &str) -> Vec<(f32, f32, f32)> {
    let split: Vec<&str> = arr.split("\n").collect();
    let mut answers: Vec<(f32, f32, f32)> = vec![];

    let str_arr: Vec<&str> = split[1].split(' ').collect();
    let mut i32_arr: Vec<i32> = vec![];
    for ele in str_arr {
        i32_arr.push(ele.parse::<i32>().expect("number here"));
    }
    answers.push(plusMinus(&i32_arr));

    answers
}

#[allow(non_snake_case)]
fn plusMinus(arr: &[i32]) -> (f32, f32, f32) {
    let n = &arr.len();

    // get ratio of pos/n, neg/n, zer/n
    let (mut pos, mut neg, mut zer) = (0, 0, 0);
    for ele in arr {
        if ele > &0 {
            pos += 1;
        } else if ele < &0 {
            neg += 1;
        } else {
            zer += 1;
        }
    }
    let pos_str = format!("{0:.6}", pos as f32 / *n as f32);
    let neg_str = format!("{0:.6}", neg as f32 / *n as f32);
    let zer_str = format!("{0:.6}", zer as f32 / *n as f32);
    println!("{}", pos_str);
    println!("{}", neg_str);
    println!("{}", zer_str);

    let answer = (
        pos_str.parse::<f32>().expect("f32"),
        neg_str.parse::<f32>().expect("f32"),
        zer_str.parse::<f32>().expect("f32"),
    );
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let test_location = "input/week1/plus_minus.txt";
        let answer: Vec<(f32, f32, f32)> = vec![(0.500000, 0.333333, 0.166667)];
        // load file or panic
        let path = String::from(test_location);
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        let my_answer = test(&input);
        assert_eq!(answer, my_answer);
    }
}
