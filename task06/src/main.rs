use std::fs;
use std::str::FromStr;

type Counts = [u64; 9];

fn default_counts() -> Counts {
    [0, 0, 0, 0, 0, 0, 0, 0, 0]
}

fn parse_count(counts: &mut Counts, input: &str) {
    input.trim().split(',').map(u8::from_str).for_each(|num| {
        match num {
            Ok(num) => {
                assert!(num < 9, "Num is out of range");
                counts[usize::from(num)] += 1;
            },
            _ => panic!("Invalid num"),
        }
    });
}

fn add_day(counts: &Counts) -> Counts {
    let mut result = default_counts();
    result[0..6].clone_from_slice(&counts[1..7]);
    result[6] = counts[0] + counts[7];
    result[7] = counts[8];
    result[8] = counts[0];
    result
}


fn main() {
    const TEST_INPUT: &str = "3,4,3,1,2";
    let test = false;
    let input = if test { String::from(TEST_INPUT) } else {
        fs::read_to_string("./input").expect("File should be there")
    };

    let mut counts: Counts = default_counts();
    parse_count(&mut counts, &input);

    for _ in 0..80 {
        counts = add_day(&counts);
    }
    println!("Part 1: {:?}", counts.iter().sum::<u64>());

    for _ in 80..256 {
        counts = add_day(&counts);
    }
    println!("Part 1: {:?}", counts.iter().sum::<u64>());
}
