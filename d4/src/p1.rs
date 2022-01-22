#![warn(clippy::pedantic)]

use d4::get_input;

fn main() {
    let mut input = get_input();

    let mut drawn: Vec<u32> = vec![];

    let mut ptr = 0;
    while input.find_winning_board(&drawn).is_none() {
        drawn.push(ptr);
        ptr += 1;
    }

    let winning_board = input.find_winning_board(&drawn).unwrap();

    // The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board.
    // Then, multiply that sum by the number that was just called when the board won to get the final score,

    let mut score = 0;
    for digit in &winning_board.digits {
        if !digit.marked {
            score += digit.num;
        }
    }

    score *= drawn.last().unwrap();

    println!("{}", score);
}
