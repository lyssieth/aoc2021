#![warn(clippy::pedantic)]

use bitvec::prelude::*;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("input");

fn main() {
    let input = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect::<Vec<_>>();

    let mut ones_in_pos: BTreeMap<usize, usize> = BTreeMap::new();
    let mut zeros_in_pos: BTreeMap<usize, usize> = BTreeMap::new();

    for item in input {
        let bits = item.view_bits::<Lsb0>();

        for (i, bit) in bits.iter().enumerate() {
            if i >= 12 {
                break;
            }
            if *bit {
                *ones_in_pos.entry(i).or_insert(0) += 1;
            } else {
                *zeros_in_pos.entry(i).or_insert(0) += 1;
            }
        }
    }

    let gamma_rate = {
        let mut build = 0usize;
        ones_in_pos
            .iter()
            .zip(zeros_in_pos.iter())
            .for_each(|(ones, zeros)| {
                if ones.1 > zeros.1 {
                    build += 1 << ones.0;
                } else {
                    build += 0 << zeros.0;
                }
            });

        build
    };

    let epsilon_rate = !gamma_rate & 0xfff;

    dbg!(gamma_rate * epsilon_rate);
}
