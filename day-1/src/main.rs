use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct DayOneAnswers {
    max_sum: usize,
    top_three_sum: usize,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let chuck_answers = chuck_fold(reader);
    dbg!(chuck_answers);
    /* chuck_answers = DayOneAnswers {
        max_sum: 69206,
        top_three_sum: 197400,
    } */

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let jaclyn_answers = jaclyn_for(reader);
    dbg!(jaclyn_answers);
    /* jaclyn_answers = DayOneAnswers {
        max_sum: 69206,
        top_three_sum: 191554,
    } */
}

fn jaclyn_for(reader: BufReader<File>) -> DayOneAnswers {
    let mut current_sum = 0;
    let mut max_sum = 0;
    let mut second_sum = 0;
    let mut third_sum = 0;

    for wrapped in reader.lines() {
        let line = wrapped.unwrap();
        if line == "" {
            current_sum = 0;
        } else {
            current_sum = current_sum + line.parse::<usize>().unwrap();
            if current_sum > max_sum {
                max_sum = current_sum;
            } else if current_sum > second_sum {
                second_sum = current_sum;
            } else if current_sum > third_sum {
                third_sum = current_sum;
            }
        }
    }

    let top_three_sum = max_sum + second_sum + third_sum;

    DayOneAnswers {
        max_sum,
        top_three_sum,
    }
}

fn chuck_fold(reader: BufReader<File>) -> DayOneAnswers {
    let (max_sum, second_sum, third_sum, _) = reader.lines().fold(
        (0, 0, 0, 0),
        |(max_sum, second_sum, third_sum, current), wrapped| {
            let line = wrapped.unwrap();
            if line == "" {
                if current > max_sum {
                    return (current, max_sum, second_sum, 0);
                } else if current > second_sum {
                    return (max_sum, current, second_sum, 0);
                } else if current > third_sum {
                    return (max_sum, second_sum, current, 0);
                }
                (max_sum, second_sum, third_sum, 0)
            } else {
                (
                    max_sum,
                    second_sum,
                    third_sum,
                    current + line.parse::<usize>().unwrap(),
                )
            }
        },
    );

    let top_three_sum = max_sum + second_sum + third_sum;

    DayOneAnswers {
        max_sum,
        top_three_sum,
    }
}
