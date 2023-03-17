use std::fs;
use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug)]
struct Board {
    numbers: [u8; 25],
    marked: u32,
}

impl Board {
    fn load(input: &str) -> Self {
        let lines = input.lines();
        let numbers: Vec<u8> = lines.flat_map(
            |line| line.split_whitespace()
                .map(|num| u8::from_str(num).expect("Lines must contain only numbers"))
        ).collect();
        assert_eq!(numbers.len(), 25, "Invalid number of numbers");
        Board {numbers: numbers.try_into().unwrap(), marked: 0 }
    }

    fn index_to_mask(&self, index: &usize) -> u32 {
        1 << (self.numbers.len() - index - 1)
    }

    fn mark(&mut self, mark: &u8) {
        self.numbers.iter().position(|&x| x == *mark)
            .map(|index| self.marked |= self.index_to_mask(&index));
    }

    fn is_done(&self) -> bool {
        for shift in 0..5 {
            let row_mask = 0b11111 << (shift * 5);
            if row_mask & self.marked == row_mask {
                return true;
            }
            let column = 0b1 << shift;
            let column_mask = column | column << 5 | column << 10 | column << 15 | column << 20;
            if column_mask & self.marked == column_mask {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.numbers.iter().enumerate()
            .filter(|(index, _)| self.marked & self.index_to_mask(index) == 0)
            .map(|(_, value)| u32::from(*value)).sum()
    }
}

fn main() {
    const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
    let test = false;
    let input = if test { String::from(TEST_INPUT) } else {
        fs::read_to_string("./input").expect("File should be there")
    };
    let mut empty_lines = input.split("\n\n");
    let guesses: Vec<u8> = empty_lines.next().unwrap().split(',').map(|s|
        u8::from_str(s).expect("Guesses must be numbers")
    ).collect();

    let mut boards: Vec<Board> = empty_lines.map(|chunk| Board::load(chunk)).collect();
    let mut finished: Vec<bool> = repeat(false).take(boards.len()).collect();
    let mut count_done = 0;

    let mut winning: Option<(usize, u8)> = None;
    let mut last_board: Option<usize> = None;
    let mut last_guess: Option<u8> = None;
    for guess in guesses {
        for index in 0..boards.len() {
            if !finished[index] {
                boards[index].mark(&guess);
                let is_done = boards[index].is_done();
                finished[index] = is_done;
                count_done += if is_done { 1 } else { 0 }
            }
        }
        if count_done == 1 && winning.is_none() {
            winning = Some((finished.iter().position(|&x| x == true).unwrap(), guess));
        }
        if count_done == boards.len() - 1 {
            last_board = Some(finished.iter().position(|&x| x == false).unwrap())
        }
        if count_done == boards.len() {
            last_guess = Some(guess);
            break;
        }
    }

    println!("Part 1: {:?}", boards[winning.unwrap().0].sum_unmarked() * u32::from(winning.unwrap().1));
    println!("Part 2: {:?}", boards[last_board.unwrap()].sum_unmarked() * u32::from(last_guess.unwrap()));
}
