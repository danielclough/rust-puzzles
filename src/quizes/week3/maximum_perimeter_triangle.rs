pub fn test(arr: &str) -> Vec<Vec<i32>> {
    let stick_vec  = read_input(arr);
    let mut answers: Vec<Vec<i32>> = vec![];
    for sticks in stick_vec {
        answers.push(maximumPerimeterTriangle(&sticks));
    };
    answers
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];
    
    // ignore % 2 == 0
    for (i, line) in lines.iter().enumerate() {
        let line_vec = line.split(" ").map(|x| x.parse::<i32>().expect("number")).collect();
        if i % 2 == 1 {
            output.push(line_vec);
        }
    }
    output
}

#[allow(non_snake_case)]
fn maximumPerimeterTriangle(sticks: &[i32]) -> Vec<i32> {
    sticks.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![
            vec![ 1,3,3 ],
            vec![ -1 ],
            vec![ 1,1,1 ],
        ];

        // load file or panic
        let path = String::from("input/week3/maximum_perimeter_triangle.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}