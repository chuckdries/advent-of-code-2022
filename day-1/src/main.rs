use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
#[derive(Debug)]
struct DayOneAnswers {
    max_sum: usize,
    top_three_sum: usize,
}

fn get_reader() -> BufReader<File> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
}

fn main() {
    let chuck_fold_answers = chuck_fold(get_reader());
    dbg!(chuck_fold_answers);

    let jaclyn_answers = jaclyn_for(get_reader());
    dbg!(jaclyn_answers);

    let chuck_for_answers = chuck_for(get_reader());
    dbg!(chuck_for_answers);
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

fn chuck_for(reader: BufReader<File>) -> DayOneAnswers {
    let mut current_sum = 0;
    let mut max_sum = 0;
    let mut second_sum = 0;
    let mut third_sum = 0;

    for wrapped in reader.lines() {
        let line = wrapped.unwrap();
        if line == "" {
            if current_sum > max_sum {
                third_sum = second_sum;
                second_sum = max_sum;
                max_sum = current_sum;
            } else if current_sum > second_sum {
                third_sum = second_sum;
                second_sum = current_sum;
            } else if current_sum > third_sum {
                third_sum = current_sum;
            }
            current_sum = 0;
        } else {
            current_sum = current_sum + line.parse::<usize>().unwrap();
        }
    }

    let top_three_sum = max_sum + second_sum + third_sum;

    DayOneAnswers {
        max_sum,
        top_three_sum,
    }
}
