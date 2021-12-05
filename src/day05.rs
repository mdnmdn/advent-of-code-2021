use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn main() {
    println!("Day 05");
    let input_file = "assets/day05/input.txt";
    let data = read_to_string(input_file).unwrap();
    let data: Vec<&str> = data.split('\n').collect();
    let solution = solve_a(&data);
    println!("> Overlapped vents {} (only square)", solution);
    let solution = solve_b(&data);
    println!("> Overlapped vents {} (also diagonal)", solution);
}

#[derive(Debug)]
struct HydrotermalVent {
    start: Point,
    end: Point,
}

#[derive(Clone, Copy)]
enum CoordinateMode {
    Square,
    SquareAndDiagonal,
}

impl HydrotermalVent {
    fn iter(&self, mode: CoordinateMode) -> CoordinateIterator {
        CoordinateIterator {
            vent: self,
            pos: 0,
            mode,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl FromStr for HydrotermalVent {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("->");

        Ok(HydrotermalVent {
            start: iter.next().unwrap().parse()?,
            end: iter.next().unwrap().parse()?,
        })
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(',');

        Ok(Point {
            x: iter.next().unwrap().parse::<i32>()?,
            y: iter.next().unwrap().parse::<i32>()?,
        })
    }
}

struct CoordinateIterator<'a> {
    vent: &'a HydrotermalVent,
    pos: i32,
    mode: CoordinateMode,
}

impl<'a> Iterator for CoordinateIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let dx = find_increment(self.vent.start.x, self.vent.end.x);
        let dy = find_increment(self.vent.start.y, self.vent.end.y);

        if let CoordinateMode::Square = self.mode {
            if dx != 0 && dy != 0 {
                return None;
            }
        }

        let x = self.vent.start.x + dx * self.pos;
        let y = self.vent.start.y + dy * self.pos;

        if (dx > 0 && x > self.vent.end.x) || (dx < 0 && x < self.vent.end.x) {
            return None;
        }

        if (dy > 0 && y > self.vent.end.y) || (dy < 0 && y < self.vent.end.y) {
            return None;
        }

        self.pos += 1;

        Some(Point { x, y })
    }
}

fn find_increment(start: i32, end: i32) -> i32 {
    match end.partial_cmp(&start).unwrap() {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        _ => 0,
    }
}

fn solve_a(data: &[&str]) -> i32 {
    solve(data, CoordinateMode::Square)
}

fn solve_b(data: &[&str]) -> i32 {
    solve(data, CoordinateMode::SquareAndDiagonal)
}

fn solve(data: &[&str], mode: CoordinateMode) -> i32 {
    let map: HashMap<i32, i32> = HashMap::new();

    let vents: Vec<HydrotermalVent> = data
        .iter()
        .map(|s| HydrotermalVent::from_str(s).unwrap())
        .collect();

    let result: HashMap<i32, i32> =
        vents
            .iter()
            .flat_map(|v| v.iter(mode))
            .fold(map, |mut m, p| {
                let key: i32 = p.x * 10000 + p.y;

                if let Some(val) = m.get_mut(&key) {
                    *val += 1;
                } else {
                    m.insert(key, 1);
                }
                m
            });

    // println!(" > {:?}", result );
    result
        .iter()
        .fold(0, |total, e| if *e.1 > 1 { total + 1 } else { total })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let data = read_to_string("assets/day05/input_sample.txt").unwrap();

        let data: Vec<&str> = data.split('\n').collect();
        let solution = solve_a(&data);
        assert_eq!(5, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = read_to_string("assets/day05/input_sample.txt").unwrap();

        let data: Vec<&str> = data.split('\n').collect();
        let solution = solve_b(&data);
        assert_eq!(12, solution);
    }

    #[test]
    fn test_parse_point() {
        let data = "5,9";
        let point = Point::from_str(data).unwrap();
        assert_eq!(5, point.x);
        assert_eq!(9, point.y);
    }
    #[test]
    fn test_parse_vent() {
        let data = "0,9 -> 5,9";
        let vent = HydrotermalVent::from_str(data).unwrap();
        assert_eq!(0, vent.start.x);
        assert_eq!(9, vent.start.y);
        assert_eq!(5, vent.end.x);
        assert_eq!(9, vent.end.y);
    }
}
