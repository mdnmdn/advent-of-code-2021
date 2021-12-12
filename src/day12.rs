use std::fs::read_to_string;

pub fn main() {
    println!("Day 12");
    let input_file = "assets/day12/input.txt";
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
//    use super::*;

//    #[test]
//    fn test_solve_a_1() {
//        let data = read_to_string("assets/day12/input_sample_01.txt").unwrap();
//
//        let data: Vec<&str> = data.split('\n').collect();
//        let solution = solve_a(&data);
//        assert_eq!(19, solution);
//    }
//
//    #[test]
//    fn test_solve_a_2() {
//        let data = read_to_string("assets/day12/input_sample_02.txt").unwrap();
//
//        let data: Vec<&str> = data.split('\n').collect();
//        let solution = solve_a(&data);
//        assert_eq!(226, solution);
//    }
}
