use std::fs::read_to_string;

pub fn main() {
    println!("Day 03");
    let input_file = "assets/day03/input.txt";
    let data = read_to_string(input_file).unwrap();
    //println!("{}", data);
    let parsed_data = parse_file(&data);
    //println!("{:?}",parsed_data);
    println!("> Read {} reports lines", parsed_data.len());
    let solution = solve_a(&parsed_data);
    println!("> Energy consumption {}", solution);
    //let solution = solve_b(&parsed_data);
    //println!("> Found {} descending sweeps with mobile average", solution);
}

fn parse_file(raw_data: &str) -> Vec<&str> {
    raw_data
        .split('\n')
        .collect()
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
        }  else {
            gamma += 1;
        }
    }

    epsilon * gamma
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
010102"#
            .split("\n")
            .collect()
    }

    #[test]
    fn test_solve() {
        let data = get_test_data();
        let solution = solve_a(&data);
        assert_eq!(198, solution);
    }

//    #[test]
//    fn test_solve_b() {
//        let data = get_test_data();
//        let solution = solve_b(&data);
//        assert_eq!(900, solution);
//    }
}
