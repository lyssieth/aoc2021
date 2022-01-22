use std::ops::Index;

#[allow(unused)]
const INPUT: &str = include_str!("input");

pub fn get_input() -> Input {
    let lines: Vec<&str> = INPUT.split("\n\n").collect();

    let numbers = lines[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let boards = lines[1..]
        .iter()
        .map(|x| {
            let line = *x;
            let mut digits: [Digit; 25] = [Digit {
                num: 0,
                marked: false,
            }; 25];
            let numbers: Vec<u32> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .take(25)
                .collect();

            for (idx, val) in numbers.iter().enumerate() {
                digits[idx] = val.into();
            }

            Board { digits }
        })
        .collect();

    Input { numbers, boards }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Input {
    pub fn find_winning_board(&mut self, drawn_numbers: &[u32]) -> Option<Board> {
        let mut res = None;

        for x in self.boards.iter_mut() {
            if x.test(drawn_numbers) {
                res.replace(*x);
                break;
            }
        }

        res
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    pub digits: [Digit; 25],
}

impl Board {
    pub fn render(&self) {
        for (idx, digit) in self.digits.iter().enumerate() {
            if idx % 5 == 0 {
                println!();
            }
            print!("{} ", digit.marked);
        }
        println!();
    }

    pub fn test(&mut self, drawn_numbers: &[u32]) -> bool {
        for x in self.digits.iter_mut() {
            if drawn_numbers.contains(&x.num) {
                x.marked = true;
            }
        }

        self.test_vertical() || self.test_horizontal() || self.test_diagonal()
    }

    // test all vertical bingo lines
    fn test_vertical(&self) -> bool {
        for j in 0..5 {
            let mut marked = 0;
            for i in 0..5 {
                if self.digits[i + j * 5].marked {
                    marked += 1;
                }
            }
            if marked == 5 {
                return true;
            }
        }

        false
    }

    // test all horizontal bingo lines
    fn test_horizontal(&self) -> bool {
        for i in 0..5 {
            let mut marked = 0;
            for j in 0..5 {
                if self.digits[i + j * 5].marked {
                    marked += 1;
                }
            }
            if marked == 5 {
                return true;
            }
        }

        false
    }

    // test all diagonal bingo lines
    fn test_diagonal(&self) -> bool {
        for i in 0..5 {
            let mut marked = 0;
            for j in 0..5 {
                if self.digits[i + j * 5].marked {
                    marked += 1;
                }
            }
            if marked == 5 {
                return true;
            }
        }

        false
    }
}

impl Index<(u8, u8)> for Board {
    type Output = Digit;

    fn index(&self, index: (u8, u8)) -> &Self::Output {
        let sum = index.0 + index.1;
        if sum > 24 {
            panic!("sum out of bounds");
        }

        &self.digits[sum as usize]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Digit {
    pub num: u32,
    pub marked: bool,
}

impl From<u32> for Digit {
    fn from(val: u32) -> Self {
        Digit {
            num: val,
            marked: false,
        }
    }
}

impl From<&u32> for Digit {
    fn from(val: &u32) -> Self {
        Digit {
            num: *val,
            marked: false,
        }
    }
}
