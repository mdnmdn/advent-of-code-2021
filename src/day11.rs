use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn main() {
    println!("Day 11");
    let input_file = "assets/day11/input.txt";
    let data = read_to_string(input_file).unwrap();
    let data = parse_file(data);
    let solution = solve_a(&data);
    println!("> Flashes after 100 iterations {}", solution);
    let solution = solve_b(&data);
    println!("> Sync at iteration {}", solution);
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

struct OctopusGrid {
    grid: Vec<u32>,
    width: usize,
    flash_counter: usize,
    generation: u32,
}

impl OctopusGrid {
    fn new(data: &[Vec<u32>]) -> Self {
        let grid = data.iter().flat_map(|v| v.iter()).copied().collect();
        OctopusGrid {
            grid,
            width: data.len(),
            flash_counter: 0,
            generation: 0,
        }
    }

    fn tick(&mut self) -> usize {
        let mut first_round = true;
        let mut flashed_octopuses: HashSet<usize> = HashSet::new();
        let mut charged_octopuses: HashMap<usize, u32> = HashMap::new();
        let height = self.grid.len() / self.width;
        self.generation += 1;
        fn charge(
            idx: usize,
            pos_x: isize,
            pos_y: isize,
            width: usize,
            height: usize,
            charged: &mut HashMap<usize, u32>,
        ) {
            let x = idx % width;

            if (x == 0 && pos_x == -1) || (x == width - 1 && pos_x > 0) {
                return;
            }

            let idx = idx as isize + pos_x + pos_y * width as isize;

            if idx < 0 || idx as usize >= (width * height) {
                return;
            }

            let charge = charged.entry(idx as usize).or_insert(0);
            *charge += 1;
        }

        loop {
            let currently_flashed = flashed_octopuses.len();
            charged_octopuses.clear();

            self.grid.iter_mut().enumerate().for_each(|(idx, val)| {
                if first_round {
                    *val += 1;
                }
                if flashed_octopuses.contains(&idx) {
                    return;
                }
                if *val > 9 {
                    flashed_octopuses.insert(idx);
                    charge(idx, -1, -1, self.width, height, &mut charged_octopuses);
                    charge(idx, 0, -1, self.width, height, &mut charged_octopuses);
                    charge(idx, 1, -1, self.width, height, &mut charged_octopuses);
                    charge(idx, 1, 0, self.width, height, &mut charged_octopuses);
                    charge(idx, 1, 1, self.width, height, &mut charged_octopuses);
                    charge(idx, 0, 1, self.width, height, &mut charged_octopuses);
                    charge(idx, -1, 1, self.width, height, &mut charged_octopuses);
                    charge(idx, -1, 0, self.width, height, &mut charged_octopuses);
                }
            });

            charged_octopuses.iter().for_each(|(idx, energy)| {
                self.grid[*idx] += *energy;
            });

            first_round = false;
            if currently_flashed == flashed_octopuses.len() {
                // no more flashes clean up
                flashed_octopuses.iter().for_each(|idx| self.grid[*idx] = 0);
                self.flash_counter += currently_flashed;
                return currently_flashed;
            }
        }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        self.grid.iter().enumerate().for_each(|(idx, v)| {
            if 0 == idx % self.width {
                println!();
            }
            print!("{:0}", v);
        });
    }
}

fn solve_a(data: &[Vec<u32>]) -> u32 {
    let mut grid = OctopusGrid::new(data);
    //grid.dump();
    (0..100).for_each(|_x| {
        grid.tick();
        // print!("\n\n Iter {} flashed {}", _x + 1, grid.flash_counter);
        // grid.dump();
    });
    grid.flash_counter as u32
}

fn solve_b(data: &[Vec<u32>]) -> u32 {
    let mut grid = OctopusGrid::new(data);
    //grid.dump();

    loop {
        if grid.tick() == grid.grid.len() {
            break;
        }
    }

    grid.generation
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_a() {
        let data = read_to_string("assets/day11/input_sample.txt").unwrap();
        let data = parse_file(data);
        let solution = solve_a(&data);
        assert_eq!(1656, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = read_to_string("assets/day11/input_sample.txt").unwrap();
        let data = parse_file(data);
        let solution = solve_b(&data);
        assert_eq!(195, solution);
    }
}
