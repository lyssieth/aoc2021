const INPUT: &str = include_str!("input");

fn main() {
    let digits = INPUT.split('\n');

    // convert char to u32s
    let digits: Vec<u32> = digits
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let windows = digits.windows(3);

    let mut counter = 0;
    let mut previous = 99999;

    for window in windows {
        let (a, b, c) = (window[0], window[1], window[2]);

        // sum up the windows and compare to previous
        let sum = a + b + c;
        if sum > previous {
            counter += 1;
        }

        previous = sum;
    }

    dbg!(counter);
}
