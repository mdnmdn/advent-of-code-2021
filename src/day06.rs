use std::fs::read_to_string;

pub fn main() {
    println!("Day 06");
    let input_file = "assets/day06/input.txt";
    let data = read_to_string(input_file).unwrap();
    let solution = solve_a(&parse_data(&data), 80);
    println!("> Lanternfish after 80 generations: {}", solution);
    let solution = solve_a(&parse_data(&data), 256);
    println!("> Lanternfish after 256 generations: {}", solution);
}

fn parse_data(data: &str) -> Vec<u64> {
    data.split(',')
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect()
}

type FishAges = [u64; 9];

fn solve_a(data: &[u64], days: u64) -> u64 {
    let mut fish_ages = FishAges::default();

    data.iter().for_each(|age| fish_ages[*age as usize] += 1);

    let mut next_fish_ages = FishAges::default();

    for _ in 0..days {
        for (i, num) in fish_ages.iter().enumerate() {
            match i {
                0 => {
                    next_fish_ages[8] += num;
                    next_fish_ages[6] += num;
                }
                _ => next_fish_ages[i - 1] += num,
            }
        }
        //let sum : u64 =next_fish_ages.iter().sum();
        //println!("> {} {:?} -> {}", gen, next_fish_ages, sum );
        std::mem::swap(&mut fish_ages, &mut next_fish_ages);
        for fishes in &mut next_fish_ages {
            *fishes = 0;
        }
    }
    fish_ages.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let data = parse_data("3,4,3,1,2");
        let solution = solve_a(&data, 80);
        assert_eq!(5934, solution);
    }
}
