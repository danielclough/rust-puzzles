pub fn test(arr: &str) {
    let lines: Vec<&str> = arr.split("\n").collect();
    //  var to indicate beginning of series
    let mut left_in_cycle = -1;
    // prepare n and q vars
    // n == ar length (reduce each round)
    let mut n = 0;
    // q == querry length  (reduce each round)
    let mut q = 0;
    // vecs to hold strings and queries
    let mut strings: Vec<String> = vec![];
    let mut queries: Vec<String> = vec![];

    for line in lines {
        if left_in_cycle == -1 {
            n = line.parse::<i32>().expect("i32 here");
            left_in_cycle = n;
        }
        else if n > 0 {
            strings.push(line.to_string());
            n -= 1;
            if n == 0 {
                left_in_cycle = -2;
            }
        }
        if left_in_cycle == -2 {
            q = line.parse::<i32>().expect("i32 here");
            left_in_cycle = q;
        }
        else if q > 0 {
            queries.push(line.to_string());
            q -= 1;
            if q == 0 {
                left_in_cycle = -1;
                // call function at end of cycle
                matchingStrings(&strings, &queries);
            }
        }
    }
    
}

#[allow (non_snake_case)]
fn matchingStrings(strings: &[String], queries: &[String]) -> Vec<i32> {
    println!("{:?}{:?}",strings, queries);
    vec![0, 5]
}