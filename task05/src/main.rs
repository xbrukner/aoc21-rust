use std::{cmp, iter};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn load(str: &str) -> Self {
        let mid = str.find(',').expect("Missing ,");
        let x = u16::from_str(&str[0..mid]).expect("Number missing");
        let y = u16::from_str(&str[mid + 1..]).expect("Number missing");
        Self { x, y }
    }
}

#[derive(Debug)]
struct MinMax{
    min: u16,
    max: u16,
}

impl MinMax {
    fn from(v1: u16, v2: u16) -> Self {
        Self {
            min: cmp::min(v1, v2),
            max: cmp::max(v1, v2),
        }
    }
    fn combine(&self, other: &Self) -> Self {
        Self {
            min: cmp::min(self.min, other.min),
            max: cmp::max(self.max, other.max)
        }
    }
    fn forwards(&self) -> impl Iterator<Item = u16> {
        (self.min ..= self.max).chain(iter::repeat(self.max))
    }
    fn backwards(&self) -> impl Iterator<Item = u16> {
        (self.min ..= self.max).rev().chain(iter::repeat(self.min))
    }
    fn size(&self) -> usize {
        usize::from(self.max - self.min + 1)
    }
}

#[derive(Debug)]
struct MinMaxLine {
    x: MinMax,
    y: MinMax,
}

impl MinMaxLine {
    fn from(line: &Line) -> Self {
        Self {
            x: MinMax::from(line.start.x, line.end.x),
            y: MinMax::from(line.start.y, line.end.y),
        }
    }

    fn combine(self, other: Self) -> Self {
        Self {
            x: MinMax::combine(&self.x, &other.x),
            y: MinMax::combine(&self.y, &other.y),
        }
    }
}


#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn load(str: &str) -> Self {
        let pattern = " -> ";
        let mid = str.find(pattern).expect("Missing -> ");
        let start = Point::load(&str[0..mid]);
        let end = Point::load(&str[mid + pattern.len()..]);
        Self { start, end }
    }

    fn vertical_or_horizontal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn generate_points<X: Iterator<Item = u16>, Y: Iterator<Item = u16>>
    (x_iter: X, y_iter: Y, size: usize) -> Vec<Point> {
        x_iter.zip(y_iter).take(size)
            .map(|(x, y)| Point { x, y })
            .collect()
    }

    fn all_points(&self) -> Vec<Point> {
        let min_max = MinMaxLine::from(self);
        let size = cmp::max(min_max.x.size(), min_max.y.size());

        // Since there is no common type, I have to expand the code.
        // Other solution would be a macro, which I do not know how to write yet.
        if self.start.x <= self.end.x {
            if self.start.y <= self.end.y {
                return Self::generate_points(min_max.x.forwards(), min_max.y.forwards(), size);
            }
            else {
                return Self::generate_points(min_max.x.forwards(), min_max.y.backwards(), size);
            }
        }
        else {
            if self.start.y <= self.end.y {
                return Self::generate_points(min_max.x.backwards(), min_max.y.forwards(), size);
            }
            else {
                return Self::generate_points(min_max.x.backwards(), min_max.y.backwards(), size);
            }
        }
    }
}

fn main() {
    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let test = false;
    let input = if test { String::from(TEST_INPUT) } else {
        fs::read_to_string("./input").expect("File should be there")
    };

    let lines: Vec<Line> = input.lines().map(Line::load).collect();
    let vertical_or_horizontal: Vec<&Line> = lines.iter()
        .filter(|&line| line.vertical_or_horizontal()).collect();
    let lines_refs: Vec<&Line> = lines.iter().collect();

    let part1 = count_intersections(&vertical_or_horizontal);
    println!("Part 1: {part1}");

    let part2 = count_intersections(&lines_refs);
    println!("Part 2: {part2}");

}

#[derive(Debug)]
struct FastIntersections {
    min_max: MinMaxLine,
    x_size: usize,
    data: Vec<u8>,
    total: usize,
}


impl FastIntersections {
    fn from(lines: &Vec<&Line>) -> Self {
        let min_max = lines
            .iter()
            .map(|&line| MinMaxLine::from(line))
            .reduce(MinMaxLine::combine)
            .unwrap();

        let x_size = min_max.x.size();
        let size = min_max.y.size() * min_max.x.size();

        Self {
            min_max,
            x_size,
            data: vec![0; size],
            total: 0
        }
    }

    fn visit(&mut self, point: &Point) {
        let index = usize::from(point.y - self.min_max.y.min) * self.x_size +
            usize::from(point.x - self.min_max.x.min);

        match self.data[index] {
            0 => self.data[index] = 1,
            1 => {
                self.data[index] = 2;
                self.total += 1
            }
            _ => {}
        }
    }
}

fn count_intersections(lines: &Vec<&Line>) -> usize {
    let mut points: HashMap<Point, u8> = HashMap::new();

    lines.iter().flat_map(|&line| line.all_points()).for_each(|point| {
        points.entry(point).and_modify(|x| *x += 1).or_insert(1);
    });

    points.into_values().filter(|&x| x > 1).count()
}

fn count_intersections_vec(lines: &Vec<&Line>) -> usize {
    let mut points: FastIntersections = FastIntersections::from(lines);

    lines.iter().flat_map(|&line| line.all_points()).for_each(|point| {
        points.visit(&point)
    });

    points.total
}
