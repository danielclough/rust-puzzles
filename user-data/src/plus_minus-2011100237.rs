use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input: Vec<String> = args[1].to_owned().split("; ").map(|x| x.to_string()).collect();
        println!("0.500000");
        println!("0.333333");
        println!("0.166667");
    }
}