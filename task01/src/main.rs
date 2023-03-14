#![feature(is_some_and)]
use std::fs;

fn main() {
    let test_input = String::from("199
200
208
210
200
207
240
269
260
263");
    let test = false;
    let input = if test { test_input } else {
        fs::read_to_string("./input").expect("File should be there")
    };
    let lines: Vec<&str> = input.split("\n").collect();
    let mut increases = 0;
    let mut previous: Option<u32> = None;
    for line in &lines {
        let num: u32 = line.parse().expect("line should be a number");
        if previous.is_some_and(|prev| prev < num) {
            increases += 1;
        }
        previous = Some(num)
    }
    let parsed= lines.iter().map(|val| val.parse().unwrap()).collect::<Vec<u32>>();
    let zipped = parsed[..].iter().zip(&parsed[1..]);
    let increases2 = zipped.filter(|(first, second)| first < second).count();
    println!("Number of increases: {increases} {increases2}");

    let sums: Vec<u32> = parsed[..].windows(3)
        .map(|window| window.iter().sum())
        .collect();
    let sum_increases = sums.iter().zip(&sums[1..])
        .filter(|(first, second)| first < second).count();
    println!("Number of windowed increases: {sum_increases}");
}
