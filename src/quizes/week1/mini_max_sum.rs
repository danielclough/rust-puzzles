pub fn test(arr: &str) {
        _ = fn_with_test_output(arr);
}

fn fn_with_test_output(arr: &str) -> Vec<String> {
    let split: Vec<&str> = arr.split("\n").collect();
    let mut answers: Vec<String> = vec![];
        
    let str_arr: Vec<&str> = split[0].split(' ').collect();
    let mut i64_arr: Vec<i64> = vec![];
    for ele in str_arr {
        i64_arr.push(ele.parse::<i64>().expect("number here"));
    };
    answers.push(miniMaxSum(&i64_arr));

    answers
}

#[allow(non_snake_case)]
fn miniMaxSum(arr: &[i64]) -> String {
    let mut sort: Vec<i64> = arr.to_owned();
    sort.sort(); 
    let mut min = 0;
    let mut max = 0;
    for (i, ele) in &mut sort.iter().enumerate() {
        if i == 0 {
            min += ele;
        } else if i == sort.len()-1  {
            max += ele;
        } else {
            min += ele;
            max += ele;
        };
    }
    let answer = format!("{} {}", min, max);
    println!{"{}", answer};
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let test_location = "input/week1/mini_max_sum.txt";
        let answer: Vec<String> = vec![
            String::from("20 20"),
        ];
        // load file or panic
        let path = String::from(test_location);
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        let my_answer = fn_with_test_output(&input);
        assert_eq!(answer, my_answer);
    }
}