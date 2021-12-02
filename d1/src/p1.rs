#![warn(clippy::pedantic)] // because it is what it is

const INPUT: &str = include_str!("input");

fn main() {
    let digits = INPUT.split('\n');

    let digits: Vec<u32> = digits
        .filter(|x| !x.is_empty()) // gotta filter because it ends with a line break, plus safety
        .flat_map(str::parse) // make it all u32s
        .collect();

    let counter = digits.windows(2).filter(|x| x[0] < x[1]).count();

    dbg!(counter); // dbg because i'm lazy
}
