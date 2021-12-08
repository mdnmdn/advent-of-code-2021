use std::fs::read_to_string;

pub fn main() {
    println!("Day 07");
    let input_file = "assets/day07/input.txt";
    let data = read_to_string(input_file).unwrap();
    let solution = solve_a(&parse_data(&data));
    println!("> Movement costs: {}", solution);
    let solution = solve_b(&parse_data(&data));
    println!("> Movement costs exp: {}", solution);
}

fn parse_data(data: &str) -> Vec<i32> {
    data.split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}

fn solve_a(data: &[i32]) -> i32 {
    let min = data.iter().min().unwrap();
    let max = data.iter().max().unwrap();
    (*min..*max)
        .into_iter()
        .map(|pos| data.iter().map(|c| (*c - pos).abs()).sum())
        .min()
        .unwrap()
}

fn solve_b(data: &[i32]) -> i32 {
    let mut fuel_costs: Vec<i32> = vec![];

    let min = data.iter().min().unwrap();
    let max = data.iter().max().unwrap();

    for v in 0..=*max {
        fuel_costs.push(match v {
            0 => 0,
            _ => v + fuel_costs[(v - 1) as usize],
        })
    }

    (*min..*max)
        .into_iter()
        .map(|pos| {
            data.iter()
                .map(|c| fuel_costs[(*c - pos).abs() as usize])
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let data = parse_data("16,1,2,0,4,2,7,1,2,14");
        let solution = solve_a(&data);
        assert_eq!(37, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = parse_data("16,1,2,0,4,2,7,1,2,14");
        let solution = solve_b(&data);
        assert_eq!(168, solution);
    }
}
