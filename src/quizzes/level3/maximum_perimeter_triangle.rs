pub fn quiz(arr: &str) -> Vec<Vec<i32>> {
    let stick_vec = read_input(arr);
    let mut answers: Vec<Vec<i32>> = vec![];
    for sticks in stick_vec {
        answers.push(maximumPerimeterTriangle(&sticks));
    }
    answers
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    // ignore % 2 == 0
    for (i, line) in lines.iter().enumerate() {
        let line_vec = line
            .split(" ")
            .map(|x| x.parse::<i32>().expect("number"))
            .collect();
        if i % 2 == 1 {
            output.push(line_vec);
        }
    }
    output
}

fn sort(vec: &mut [i32]) -> Vec<i32> {
    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
    vec.to_owned()
}
// Choose triangle with the longest maximum side.
// The longest minimum side breaks tie.
// If no non-degenerate triangle exists, return -1.

// VALID IF
// a + b > c
// a + c > b
// b + c > a

#[allow(non_snake_case)]
fn check_tri_valid(mut sorted: Vec<i32>) -> Vec<i32> {
    let C = sorted.pop().unwrap();
    let B = sorted.pop().unwrap();
    let mut A = sorted.pop().unwrap();

    let is_valid = A + B > C && A + C > B && B + C > A;

    while !is_valid && sorted.len() > 0 {
        A = sorted.pop().unwrap();
    }

    let mut answer = vec![-1];
    if is_valid {
        answer = vec![A, B, C]
    }
    answer
}

fn check_cur_order(mut sorted: Vec<i32>) -> Vec<i32> {
    let mut answer = vec![-1];
    let mut counter = 0;
    while counter == 0 || (answer.len() == 1 && sorted.len() > 3) {
        if counter != 0 {
            _ = sorted.pop()
        };
        answer = check_tri_valid(sorted.to_owned());
        counter += 1;
    }
    answer
}

#[allow(non_snake_case)]
fn maximumPerimeterTriangle(sticks: &[i32]) -> Vec<i32> {
    let mut answer = vec![-1];

    let mut sorted = sort(&mut sticks.to_owned());

    println!("  {:?}\n  {:?}", sorted, answer);

    let mut counter = 0;
    while counter == 0 || (counter < sorted.len() && answer.len() == 1 && sorted.len() > 3) {
        answer = check_cur_order(sorted.to_owned());
        sorted.rotate_right(1);
        counter += 1;
    }
    println!("{:?}", answer);

    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let answer = vec![vec![1, 3, 3], vec![-1], vec![1, 1, 1], vec![2, 3, 3]];

        // load file or panic
        let path = "input/level3/maximum_perimeter_triangle.txt";
        let input = fs::read_to_string(path).unwrap();

        assert_eq!(answer, quiz(&input));
    }
}