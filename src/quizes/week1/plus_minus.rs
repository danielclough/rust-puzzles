pub fn test(arr: &str) {
    let split: Vec<&str> = arr.split("\n").collect();
    
    let str_arr: Vec<&str> = split[1].split(' ').collect();
    let mut i32_arr: Vec<i32> = vec![];
    for ele in str_arr {
        i32_arr.push(ele.parse::<i32>().expect("number here"));
    };
    plusMinus(&i32_arr);
}

#[allow(non_snake_case)]
fn plusMinus(arr: &[i32]) {
    let n = &arr.len();
    
    // get ratio of pos/n, neg/n, zer/n
    let (mut pos,mut neg,mut zer) = (0, 0 , 0);
    for ele in arr {
        if  ele > &0 {
            pos+=1;
        }
        else if ele < &0 {
            neg+=1;
        }
        else {
            zer+=1;
        }
    }
    println!("{0:.6}", pos as f32 / *n as f32);
    println!("{0:.6}", neg as f32 / *n as f32);
    println!("{0:.6}", zer as f32 / *n as f32);    
}