use std::fs::read_to_string;

pub fn main() {
    let input_file = "assets/day01/input.txt";
    let data = read_to_string(input_file).unwrap();
    //println!("{}", data);
    let parsed_data = parse_file(data);
    //println!("{:?}",parsed_data);
    println!("Read {} sweeps", parsed_data.len());
    let solution = solve_a(&parsed_data);
    println!("Found {} descending sweeps", solution);
    let solution = solve_b(&parsed_data);
    println!("Found {} descending sweeps with mobile average", solution);
}

fn parse_file(raw_data: String) -> Vec<i32> {
    raw_data
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn solve_a(data: &Vec<i32>) -> usize {
    let mut ahead_iter = data.iter();
    ahead_iter.next();
    ahead_iter.zip(data.iter()).filter(|i| i.0 > i.1).count()
}

fn solve_b(data: &Vec<i32>) -> usize {
    #[derive(Debug)]
    struct Acc {
        sliding_values: [i32; 3],
        previous_value: i32,
        position: usize,
        sweeps: usize,
    }

    let zero_acc = Acc {
        sliding_values: [0, 0, 0],
        previous_value: 0,
        position: 0,
        sweeps: 0,
    };

    let result = data.iter().fold(zero_acc, |mut acc, x| {
        //println!("{:?}", acc);
        acc.sliding_values[acc.position % 3] = *x;
        acc.position = acc.position + 1;
        let sum = acc.sliding_values[0] + acc.sliding_values[1] + acc.sliding_values[2];
        if acc.position > 3 && sum > acc.previous_value {
            acc.sweeps = acc.sweeps + 1;
        }
        acc.previous_value = sum;

        acc
    });
    result.sweeps
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_data() -> Vec<i32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_solve() {
        let data = get_test_data();
        let solution = solve_a(&data);
        assert_eq!(7, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = get_test_data();
        let solution = solve_b(&data);
        assert_eq!(5, solution);
    }
}
