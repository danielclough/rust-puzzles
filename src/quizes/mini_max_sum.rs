pub fn test(arr: &str) {
    let split: Vec<&str> = arr.split("\n").collect();
    
    let str_arr: Vec<&str> = split[0].split(' ').collect();
    let mut i64_arr: Vec<i64> = vec![];
    for ele in str_arr {
        i64_arr.push(ele.parse::<i64>().expect("number here"));
    };
    miniMaxSum(&i64_arr);
}

#[allow(non_snake_case)]
fn miniMaxSum(arr: &[i64]) {
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
    println!{"{} {}", min, max};
}
