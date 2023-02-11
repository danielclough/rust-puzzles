pub fn quiz(arr: &str) -> Vec<i32> {
    let ints = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for arr in ints {
        answers.push(lonelyinteger(&arr));
    }
    answers
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        let str_arr: Vec<&str> = line.split(' ').collect();
        let mut i32_arr: Vec<i32> = vec![];
        for ele in str_arr {
            i32_arr.push(ele.parse::<i32>().expect("number here"));
        }

        if i % 2 == 1 {
            output.push(i32_arr);
        }
    }
    output
}
#[derive(PartialEq, Debug, Clone, Copy)]
struct Has {
    int: i32,
    n: i32,
}

// Input == int arr
// Output == unique int in arr
fn lonelyinteger(a: &[i32]) -> i32 {
    let mut a_has: Vec<Has> = vec![];
    for int_from_a in a {
        if a_has.contains(&Has {
            int: int_from_a.to_owned(),
            n: 1,
        }) {
            let index = a_has
                .iter()
                .position(|x| x.int == int_from_a.to_owned())
                .unwrap();
            a_has[index].n += 1;
            println!("Index: {}", index);
        } else {
            a_has.push(Has {
                int: int_from_a.to_owned(),
                n: 1,
            });
            println!("a_has: {:?}", a_has);
        }
    }
    let answer_index = a_has.iter().position(|x| x.n == 1).unwrap();
    let answer: Has = a_has[answer_index];
    answer.int
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work() {
        let answer = vec![1, 2, 2];

        // load file or panic
        let path = "input/week2/lonely_integer.txt";
        let input = fs::read_to_string(path).unwrap();

        assert_eq!(answer, quiz(&input));
    }
}
