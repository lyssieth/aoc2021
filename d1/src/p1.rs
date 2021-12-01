#![warn(clippy::pedantic)] // because it is what it is

const INPUT: &str = include_str!("input");

fn main() {
    let digits = INPUT.split('\n');

    let digits: Vec<u32> = digits
        .filter(|x| !x.is_empty()) // gotta filter because it ends with a line break, plus safety
        .map(|x| x.parse::<u32>().unwrap()) // make it all u32s
        .collect();

    let mut counter = 0;
    let mut previous = 99999; // 99999 because the first number doesn't matter
    for x in digits {
        if x > previous {
            counter += 1;
        } else {
        }

        previous = x;
    }

    dbg!(counter); // dbg because i'm lazy
}
