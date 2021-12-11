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

fn solve_b(data: &[Vec<u32>]) -> u32 {
    //let (width, height) = (data[0].len(), data.len());

    #[derive(Default, Debug)]
    struct BasinRegistry {
        basins_weight: HashMap<u32, u32>,
        next_basin: u32,
        //current_basin: Option<u32>,
        connected_basins: HashMap<u32, u32>,
        basin_map: Vec<Vec<Option<u32>>>,
    }

    let mut registry = BasinRegistry::default();

    for (r_pos, row) in data.iter().enumerate() {
        registry.basin_map.push(Vec::new());
        for (c_pos, val) in row.iter().enumerate() {
            let mut current_basin: Option<u32> = None;
            if *val == 9 {
                //registry.current_basin = None;
                registry.basin_map[r_pos].push(None);
                continue;
            }
            if r_pos > 0 {
                if let Some(top_basin) = registry.basin_map[r_pos - 1].get(c_pos) {
                    current_basin = *top_basin;
                }
            }

            if c_pos > 0 {
                if let Some(Some(left_basin)) = registry.basin_map[r_pos].get(c_pos - 1) {
                    if let Some(top_basin) = current_basin {
                        if top_basin != *left_basin {
                            // manage connected basins
                            current_basin = Some(top_basin);

                            let dest_basin =
                                find_fist_basin(&registry.connected_basins, &top_basin);
                            registry.connected_basins.insert(*left_basin, dest_basin);
                        }
                    } else {
                        current_basin = Some(*left_basin);
                    }
                }
            }
            if current_basin.is_some() {
                current_basin = Some(registry.next_basin);
                registry.next_basin += 1;
            }

            let weight = registry
                .basins_weight
                .entry(current_basin.unwrap())
                .or_insert(0);
            *weight += 1;

            registry.basin_map[r_pos].push(current_basin);
        }
    }
    println!("{:?}", registry.connected_basins);

    let mut connections: HashMap<u32, u32> = HashMap::new();

    registry.connected_basins.iter().for_each(|(k, v)| {
        println!(" > {} -> {}", k, v);
        let root_basin = find_fist_basin(&registry.connected_basins, v);
        connections.insert(*k, root_basin);
        let weight_to_relocate = *registry.basins_weight.get(k).unwrap();
        let weight = registry.basins_weight.get_mut(&root_basin).unwrap();
        *weight += weight_to_relocate;
        registry.basins_weight.remove(k);
    });
    println!("{:?}", registry);

    let mut values = registry.basins_weight.values().collect::<Vec<_>>();
    values.sort();
    values.reverse();

    println!("{:?}", values);

    print_map(&registry.basin_map, &connections, &registry.basins_weight);
    print!(
        "\n\n------------\n{:?}\n---------",
        registry.connected_basins
    );
    print!("\n\n------------\n{:?}\n---------", connections);
    values.into_iter().take(3).fold(1, |mut r, v| {
        r *= *v;
        r
    })
}

fn find_fist_basin(connections: &HashMap<u32, u32>, basin: &u32) -> u32 {
    let mut dest = basin;

    while let Some(new_basin) = connections.get(dest) {
        dest = new_basin;
    }
    *dest
}

fn print_map(
    map: &[Vec<Option<u32>>],
    connections: &HashMap<u32, u32>,
    weights: &HashMap<u32, u32>,
) {
    println!("\n\n\n\n----------------------------");
    for r in map {
        for v in r {
            if let Some(idx) = v {
                let mut real_val = find_fist_basin(connections, idx);
                real_val = *weights.get(&real_val).unwrap();

                print!("{:3}", real_val);
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
