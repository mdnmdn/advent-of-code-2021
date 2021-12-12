use std::collections::HashMap;
use std::fs::read_to_string;

pub fn main() {
    println!("Day 10");
    let input_file = "assets/day10/input.txt";
    let data = read_to_string(input_file).unwrap();
    let data: Vec<&str> = data.split('\n').collect();
    let solution = solve_a(&data);
    println!("> Error points {}", solution);
}

#[derive(Debug)]
struct InstructionInfo {
    open: char,
    close: char,
    error_points: u32,
}

#[derive(Debug)]
struct InstructionParser {
    error_points: u32,
    stack: Vec<char>,
    points: HashMap<char, InstructionInfo>,
}

impl Default for InstructionParser {
    fn default() -> Self {
        let mut points: HashMap<char, InstructionInfo> = HashMap::new();
        points.insert(
            ')',
            InstructionInfo {
                open: '(',
                close: ')',
                error_points: 3,
            },
        );
        points.insert(
            ']',
            InstructionInfo {
                open: '[',
                close: ']',
                error_points: 57,
            },
        );
        points.insert(
            '}',
            InstructionInfo {
                open: '{',
                close: '}',
                error_points: 1197,
            },
        );
        points.insert(
            '>',
            InstructionInfo {
                open: '<',
                close: '>',
                error_points: 25137,
            },
        );
        InstructionParser {
            points,
            error_points: 0,
            stack: Vec::default(),
        }
    }
}

const OPENING_CHARS: [char; 4] = ['(', '{', '<', '['];

impl InstructionParser {
    fn process(&mut self, instr: char) {
        if OPENING_CHARS.contains(&instr) {
            self.stack.push(instr);
        } else {
            let instruction = self.points.get(&instr).unwrap();
            let last = self.stack.last();
            if last.is_none() || *last.unwrap() != instruction.open {
                //println!("err: {} {:?}-> {:?}", instr, last, self.stack);
                self.error_points += instruction.error_points;
            };

            if last.is_some() {
                self.stack.pop();
            }
        }
    }
}

fn solve_a(data: &[&str]) -> u32 {
    data.iter()
        .map(|r| {
            r.chars()
                .into_iter()
                .fold(InstructionParser::default(), |mut parser, instr| {
                    parser.process(instr);
                    parser
                })
                .error_points
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_a() {
        let data = read_to_string("assets/day10/input_sample.txt").unwrap();
        let data: Vec<&str> = data.split('\n').collect();
        let solution = solve_a(&data);
        assert_eq!(26397, solution);
    }

    //    #[test]
    //    fn test_solve_b() {
    //        let data = read_to_string("assets/day10/input_sample.txt").unwrap();
    //        let data: Vec<&str> = data.split('\n').collect();
    //        let solution = solve_b(&data);
    //        assert_eq!(288957, solution);
    //    }
}
