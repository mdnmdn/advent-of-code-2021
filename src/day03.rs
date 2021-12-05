use crate::metrics::AllocatorCounter;
use std::cmp::Ordering;
use std::default::Default;
use std::fs::read_to_string;

pub fn main() {
    println!("Day 03");
    let input_file = "assets/day03/input.txt";
    let data = read_to_string(input_file).unwrap();
    //println!("{}", data);
    let parsed_data = parse_file(&data);
    let ac = AllocatorCounter::default();
    //println!("{:?}",parsed_data);
    println!("> Read {} reports lines", parsed_data.len());
    let solution = solve_a(&parsed_data);
    let allocations = ac.count();
    println!("> Energy consumption {}", solution);
    println!("> Allocations {}", allocations);

    let r = ac.measure(|| solve_b(&parsed_data));
    let solution = r.result;
    println!("> Life support rating {}", solution);
    println!("> Allocations {}", r.allocations);
}

fn parse_file(raw_data: &str) -> Vec<&str> {
    raw_data.split('\n').collect()
}

fn solve_a(data: &[&str]) -> u32 {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for pos in 0..data[0].len() {
        let zeros = data
            .iter()
            .filter(|s| s.chars().nth(pos).unwrap() == '0')
            .count();
        gamma <<= 1;
        epsilon <<= 1;
        if zeros > data.len() / 2 {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }

    epsilon * gamma
}

fn solve_b(data: &[&str]) -> u32 {
    let mut oxy: u32 = 0;
    let mut co2: u32 = 0;

    let mut oxy_codes = Vec::from(data);
    let mut co2_codes = Vec::from(data);

    for pos in 0..data[0].len() {
        oxy <<= 1;
        let occurrences = find_max_occurrences(&oxy_codes, pos);

        let filter_char_oxy = match occurrences {
            Some('0') => '0',
            _ => {
                oxy += 1;
                '1'
            }
        };

        oxy_codes = filter_codes(oxy_codes, pos, filter_char_oxy);

        let occurrences = find_max_occurrences(&co2_codes, pos);

        co2 <<= 1;
        let filter_char_co2 = match occurrences {
            Some('0') => {
                if co2_codes.len() > 1 {
                    co2 += 1;
                }
                '1'
            }
            _ => {
                if co2_codes.len() == 1 {
                    co2 += 1;
                }
                '0'
            }
        };

        co2_codes = filter_codes(co2_codes, pos, filter_char_co2);

        //println!(
        //    "{} - oxy: {}     {} -> {:?}",
        //    pos, oxy, filter_char_oxy, oxy_codes
        //);
        //println!(
        //    "{} - co2: {} {:b}    {} -> {:?}",
        //    pos, co2, co2,filter_char_co2, co2_codes
        //);
    }

    oxy * co2
}

fn find_max_occurrences(codes: &[&str], pos: usize) -> Option<char> {
    #[derive(Default)]
    struct Acc {
        zeros: u32,
        ones: u32,
    }

    let res = codes.iter().fold(Acc::default(), |mut acc, c| {
        match c.chars().nth(pos).unwrap() {
            '1' => acc.ones += 1,
            '0' => acc.zeros += 1,
            _ => {}
        }
        acc
    });

    match res.ones.partial_cmp(&res.zeros)? {
        Ordering::Greater => Some('1'),
        Ordering::Less => Some('0'),
        _ => None,
    }
}

fn filter_codes(codes: Vec<&str>, pos: usize, val: char) -> Vec<&str> {
    if codes.len() == 1 {
        codes
    } else {
        codes
            .into_iter()
            .filter(|&c| c.chars().nth(pos).unwrap() == val)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> Vec<&'static str> {
        r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#
            .split("\n")
            .collect()
    }

    #[test]
    fn test_solve() {
        let data = get_test_data();
        let solution = solve_a(&data);
        assert_eq!(198, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = get_test_data();
        let solution = solve_b(&data);
        assert_eq!(230, solution);
    }
}
