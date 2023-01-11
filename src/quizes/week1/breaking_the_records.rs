pub fn test(arr: &str) {
    _ = the_thing_itself(arr);
}

fn the_thing_itself(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output: Vec<Vec<i32>> = vec![];
    
    for (i, line) in lines.iter().enumerate() {
        let str_arr: Vec<&str> = line.split(' ').collect();
        let mut i32_arr: Vec<i32> = vec![];
        for ele in str_arr {
            i32_arr.push(ele.parse::<i32>().expect("number here"));
        };

        if i%2 == 1 {
            output.push(breakingRecords(&i32_arr));
        }
    }
    output
}

#[allow(non_snake_case)]
fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    // let n = &scores.len();
    let mut current_min = 0;
    let mut min = 0;
    let mut current_max = 0;
    let mut max = 0;
    for (i,score) in scores.iter().enumerate() {
        if i == 0 {
            current_min = score.to_owned();
            current_max = score.to_owned();
        }
        else if score < &current_min {
            min+=1;
            current_min = score.to_owned();
    println!("min: {} {} {}",score,min,current_min);
        } else if score > &current_max {
            max+=1;
            current_max = score.to_owned();
    println!("max: {} {} {}",score,max,current_max);
        }
    }
    let mut return_vec: Vec<i32> = vec![];
    return_vec.push(max);
    return_vec.push(min);

    println!("{} {}",max,min);
    println!("{:?}",return_vec);
    return_vec

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let answer: Vec<Vec<i32>> = vec![
            vec![2, 4],
            vec![4, 0],
        ];
        
        // load file or panic
        let path = String::from("input/week1/breaking_the_records.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");

        assert_eq!(answer, the_thing_itself(&input));
    }
}