use std::collections::HashMap;
use std::fs::read_to_string;

pub fn main() {
    println!("Day 09");
    let input_file = "assets/day09/input.txt";
    let data = read_to_string(input_file).unwrap();
    let parsed_data = parse_file(data);
    let solution = solve_a(&parsed_data);
    println!("> Risk points {}", solution);
    let solution = solve_b(&parsed_data);
    println!("> Largest basins {}", solution);
}

fn parse_file(raw_data: String) -> Vec<Vec<u32>> {
    raw_data
        .split('\n')
        .map(|s| {
            s.chars()
                .filter(|c| c.is_numeric())
                .map(|c| (c as u32 - '0' as u32))
                .collect()
        })
        .collect()
}

fn solve_a(data: &[Vec<u32>]) -> u32 {
    let mut res: u32 = 0;
    let (width, height) = (data[0].len(), data.len());
    for (r_pos, row) in data.iter().enumerate() {
        for (c_pos, val) in row.iter().enumerate() {
            if c_pos > 0 && val >= &row[c_pos - 1] {
                continue;
            }
            if c_pos < width - 1 && val >= &row[c_pos + 1] {
                continue;
            }
            if r_pos > 0 && val >= &data[r_pos - 1][c_pos] {
                continue;
            }
            if r_pos < height - 1 && val >= &data[r_pos + 1][c_pos] {
                continue;
            }
            //println!("{}x{} -> {}", r_pos, c_pos, val);
            res += val + 1;
        }
    }
    res
}

#[derive(Default, Debug)]
struct BasinRegistry<'a> {
    basins_weight: HashMap<u32, u32>,
    basin_map: Vec<Vec<Option<u32>>>,
    map: &'a [Vec<u32>],
}

impl<'a> BasinRegistry<'a> {
    fn new(map: &'a [Vec<u32>]) -> Self {

        let mut result = BasinRegistry {
            basins_weight: HashMap::default(),
            basin_map: Vec::default(),
            map,
        };
        let (width, height) = (map[0].len(), map.len());

        (0..height).into_iter().for_each(|_| {
            result.basin_map.push(
                (0..width).into_iter().map(|_| None).collect()
            );
        });

        result
    }

    fn explore(&mut self, x: usize, y: usize) -> u32 {
        let basin_val = self.basin_map.get(x).unwrap().get(y).unwrap();
        if basin_val.is_none()  {
            let new_basin = self.basins_weight.len() as u32;
            println!("explore({}, {}, {}) ",x, y, new_basin);
            let weight = self.visit(x, y, &new_basin);
            self.basins_weight.insert(new_basin, weight);
            weight
        } else {
            0
        }
    }

    fn visit(&mut self, x: usize, y: usize, basin: &u32) -> u32 {
        println!("visit({}, {}, {}) ",x, y, basin);
        if let Some(Some(v)) = self.basin_map.get(x).unwrap().get(y) {
            if *v == *basin {
                return 0;
            } else {
                panic!("Error: visiting: {},{} for basin {}, but is already occupied by {} ",
                    x, y, *basin, *v);
            }
        }

        let val = self.map[x][y];
        if val == 9 {
            return 0;
        }
        self.basin_map.get_mut(x).unwrap()[y] = Some(*basin);
        let width = self.basin_map.get(x).unwrap().len();
        let mut weight = 1;
        if x > 0 {
            weight += self.visit(x - 1, y, basin);
        }

        if x < self.basin_map.len() - 1  {
            weight += self.visit(x + 1, y, basin);
        }

        if y > 0 {
            weight += self.visit(x, y - 1, basin);
        }

        if y < width - 1  {
            weight += self.visit(x, y + 1, basin);
        }

        weight
    }
}


fn solve_b(data: &[Vec<u32>]) -> u32 {
    let mut registry = BasinRegistry::new(data);

    for (r_pos, row) in data.iter().enumerate() {
        for (c_pos, val) in row.iter().enumerate() {
            if *val != 9 {
                registry.explore(r_pos, c_pos);
            }
        }
    }

    println!("{:?}", registry);

    let mut values = registry.basins_weight.values().collect::<Vec<_>>();
    values.sort();
    values.reverse();

    println!("{:?}", values);

    print_map(&registry.basin_map, &registry.basins_weight);

    values.into_iter().take(3).fold(1, |mut r, v| {
        r *= *v;
        r
    })
}


fn print_map(
    map: &[Vec<Option<u32>>],
    _weights: &HashMap<u32, u32>,
) {
    println!("\n\n\n\n----------------------------");
    for r in map {
        for v in r {
            if let Some(idx) = v {
                //let real_val = *weights.get(&real_val).unwrap();
                print!("{:3}", idx);
            } else {
                print!("   ");
            }
        }
    }
    println!("----------------------------\n\n\n");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_a() {
        let data = read_to_string("assets/day09/input_sample.txt").unwrap();
        let parsed_data = parse_file(data);
        let solution = solve_a(&parsed_data);
        assert_eq!(15, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = read_to_string("assets/day09/input_sample.txt").unwrap();
        let parsed_data = parse_file(data);
        let solution = solve_b(&parsed_data);
        assert_eq!(1134, solution);
    }
}
