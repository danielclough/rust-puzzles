use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    quiz(args[1].to_owned().split("; ").map(|x| x.to_string()).collect());
}
// Calculate ratios as positive, negative, or zero.
// Given n integers, find the ratio of the total which are positive, negative, or zero with a precision of 6 decimal places.
// ["0 < n <= 100", "-100 <= arr[n] <= 100"]

fn quiz(input: Vec<String>) {
    println!("Input: {:?}", input);
}