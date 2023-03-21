use std::cmp::{max, min};
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Crabs {
    counts: Vec<u8>,
}

impl Crabs {
    fn load_input(input: &str) -> Crabs {
        let numbers: Vec<usize> = input.trim().split(',').map(usize::from_str)
            .map(|num| num.unwrap()).collect();

        let size = numbers.iter().reduce(Ord::max).unwrap();
        let mut counts = vec![0; *size + 1];
        for num in numbers {
            counts[num] += 1;
        }

        Crabs { counts, }
    }

    fn calculate_move_cost_linear(&self, to: usize) -> usize {
        self.counts.iter().enumerate().fold(0,
                                               |acc, (index, &count)| {
            acc + (max(to, index) - min(to, index)) * usize::from(count)
        })
    }

    fn calculate_move_cost_increasing(&self, to: usize) -> usize {
        fn increasing_cost(max: usize, min: usize) -> usize {
            match max - min {
                0 => 0,
                1 => 1,
                _ => (max - min) * (max - min + 1) / 2
            }
        }
        self.counts.iter().enumerate().fold(0,
                                            |acc, (index, &count)| {
                                                acc + increasing_cost(max(to, index), min(to, index)) * usize::from(count)
                                            })
    }
}


fn main() {
    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    let test = false;
    let input = if test { String::from(TEST_INPUT) } else {
        fs::read_to_string("./input").expect("File should be there")
    };
    let crabs = Crabs::load_input(&input);

    let align_cost_part1 = (0..crabs.counts.len())
        .map(|index| crabs.calculate_move_cost_linear(index))
        .reduce(min).unwrap();
    let align_cost_part2 = (0..crabs.counts.len())
        .map(|index| crabs.calculate_move_cost_increasing(index))
        .reduce(min).unwrap();
    println!("Part 1: {align_cost_part1}");
    println!("Part 2: {align_cost_part2}");
}
