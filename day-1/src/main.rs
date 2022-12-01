use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let (max_sum, second_sum, third_sum, _) = reader.lines().fold(
        (0, 0, 0, 0),
        |(max_sum, second_sum, third_sum, current), wrapped| {
            let line = wrapped.unwrap();
            if line == "" {
                if current > max_sum {
                    return (current, second_sum, third_sum, 0);
                } else if current > second_sum {
                    return (max_sum, current, third_sum, 0);
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

    println!("highest calories: {max_sum}");

    println!("sums: {max_sum} {second_sum} {third_sum}");

    let top_three = max_sum + second_sum + third_sum;

    println!("top three calories: {}", top_three);
}
