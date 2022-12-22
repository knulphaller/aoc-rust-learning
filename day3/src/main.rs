use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let ruck = input.split('\n').into_iter().collect::<Vec<_>>();
    let mut mymap = HashMap::new();

    let vz = (97..=122).chain(65..=90).collect::<Vec<u8>>();

    for (index, number) in vz.iter().enumerate() {
        mymap.insert(number, index + 1);
    }

    let s: i32 = ruck
        .iter()
        .map(|i| i32::try_from(*mymap.get(&duplicate_finder(i)).unwrap()).unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("{s:?}");
}

fn duplicate_finder(s: &str) -> u8 {
    let half = s.len() / 2;
    let first_part: HashSet<u8> = s.bytes().take(half).clone().collect();
    let second_part: HashSet<u8> = s.bytes().rev().take(half).collect();
    let diff: HashSet<_> = first_part.intersection(&second_part).collect();
    diff.into_iter().collect::<Vec<&u8>>()[0].clone()
}
