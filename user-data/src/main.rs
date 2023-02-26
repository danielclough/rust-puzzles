use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    quiz(args[1].to_owned().split("; ").map(|x| x.to_string()).collect());
}
// Find the min and max values by summing four of five given integers.
// Print the min and max values as a single line separated by a space.
// ["1 <= arr[i] <= 10^9"]

fn quiz(input: Vec<String>) {
    println!("Input: {:?}", input);
}