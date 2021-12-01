#![warn(clippy::pedantic)]

const INPUT: &str = include_str!("input");

fn main() {
    let digits = INPUT.split('\n');

    // convert char to u32s
    let digits: Vec<u32> = digits
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // count the number of times the previous number is smaller than the current number
    let mut counter = 0;
    let mut previous = 0;
    for x in digits {
        if previous == 0 {
            previous = x;
            continue;
        }
        if x > previous {
            println!("{} (increased)", x);
            counter += 1;
        } else {
            println!("{} (decreased)", x);
        }

        previous = x;
    }

    dbg!(counter);
}
