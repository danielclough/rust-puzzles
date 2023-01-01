pub fn test(arr: &str) {
    let split: Vec<&str> = arr.split("\n").collect();
    
    let str_arr: Vec<&str> = split[0].split(' ').collect();
    let mut i32_arr: Vec<i32> = vec![];
    for ele in str_arr {
        i32_arr.push(ele.parse::<i32>().expect("number here"));
    };
    miniMaxSum(&i32_arr);
}

#[allow(non_snake_case)]
fn miniMaxSum(arr: &[i32]) {
    for ele in arr {
        println!("{ele}")
    }
}
