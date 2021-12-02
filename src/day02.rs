use std::fs::read_to_string;

pub fn main() {
    println!("Day 02");
    let input_file = "assets/day02/input.txt";
    let data = read_to_string(input_file).unwrap();
    //println!("{}", data);
    let parsed_data = parse_file(data);
    //println!("{:?}",parsed_data);
    println!("> Read {} commands", parsed_data.len());
    let solution = solve_a(&parsed_data);
    println!("> Movement area: {}", solution);
    let solution = solve_b(&parsed_data);
    println!("> Movement area with aim: {}", solution);
}

fn parse_file(raw_data: String) -> Vec<String> {
    raw_data.split('\n').map(|s| s.to_string()).collect()
}

fn solve_a(data: &[String]) -> u32 {
    struct Acc {
        depth: u32,
        distance: u32,
    }

    let acc_zero = Acc {
        depth: 0,
        distance: 0,
    };

    let result = data
        .iter()
        .map(|s| s.split_terminator(char::is_whitespace))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap().parse::<u32>().unwrap()))
        .fold(acc_zero, |mut acc, v: (&str, u32)| {
            match v.0 {
                "forward" => acc.distance += v.1,
                "down" => acc.depth += v.1,
                "up" => acc.depth -= v.1,
                _ => {}
            }
            acc
        });

    result.depth * result.distance
}

fn solve_b(data: &[String]) -> u32 {
    #[derive(Debug)]
    struct Acc {
        depth: u32,
        distance: u32,
        aim: u32,
    }

    let acc_zero = Acc {
        depth: 0,
        distance: 0,
        aim: 0,
    };

    let result = data
        .iter()
        .map(|s| s.split_terminator(' '))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap().parse::<u32>().unwrap()))
        .fold(acc_zero, |mut acc, v: (&str, u32)| {
            match v.0 {
                "forward" => {
                    acc.distance += v.1;
                    acc.depth += acc.aim * v.1;
                }
                "down" => acc.aim += v.1,
                "up" => acc.aim -= v.1,
                _ => {}
            }
            //println!("> {:?}  - {:?}", acc,v);
            acc
        });

    result.depth * result.distance
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> Vec<String> {
        r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_solve() {
        let data = get_test_data();
        let solution = solve_a(&data);
        assert_eq!(150, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = get_test_data();
        let solution = solve_b(&data);
        assert_eq!(900, solution);
    }
}
