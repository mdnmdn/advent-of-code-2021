use std::collections::HashMap;
use std::fs::read_to_string;

pub fn main() {
    println!("Day 11");
    let input_file = "assets/day11/input.txt";
    let data = read_to_string(input_file).unwrap();
    let data: Vec<&str> = data.split('\n').collect();
    let solution = solve_a(&data);
    println!("> Error points {}", solution);
}

fn solve_a(_data: &[&str]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_a() {
        let data = read_to_string("assets/day11/input_sample.txt").unwrap();

        let data: Vec<&str> = data.split('\n').collect();
        let solution = solve_a(&data);
        assert_eq!(1656, solution);
    }
}
