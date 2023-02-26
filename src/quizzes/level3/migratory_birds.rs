use crate::quizzes::{types::QuizConfig, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "migratory_birds".to_string(),
        level: "level3".to_string(),
    };
    output
}


pub fn quiz(arr: &str) -> Vec<i32> {
    let inputs = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for input in inputs {
        answers.push(migratoryBirds(&input));
    }
    answers
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        if i % 2 == 1 {
            output.push(
                line.split(" ")
                    .map(|x| x.parse::<i32>().expect("number"))
                    .collect(),
            );
        }
    }
    output
}

#[derive(Clone, Debug)]
struct Bird {
    id: i32,
    sightings: i32,
}
impl Bird {
    fn new() -> Bird {
        Bird {
            id: 0,
            sightings: 0,
        }
    }
}

#[allow(non_snake_case)]
fn migratoryBirds(arr: &[i32]) -> i32 {
    let n_of_birds = 5;
    // init birds vec
    let mut birds: Vec<Bird> = vec![Bird::new(); n_of_birds];
    let mut counter = 0;
    for b in birds.iter_mut() {
        b.id = (counter + 1) as i32;
        counter += 1;
    }
    // arr of sightings
    for id in arr {
        for b in birds.iter_mut() {
            if b.id == id.to_owned() {
                b.sightings += 1;
            }
        }
    }
    // id, sightings
    let mut most_sighted = (0, 0);
    for b in birds {
        if b.sightings > most_sighted.1 {
            most_sighted.0 = b.id;
            most_sighted.1 = b.sightings;
        }
    }

    println!("{:?}", most_sighted);
    most_sighted.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer = vec![4, 3];
        let config = config();
        let input = read_from_input_file(&config.level, &config.name);

        assert_eq!(answer, quiz(&input));
    }
}
