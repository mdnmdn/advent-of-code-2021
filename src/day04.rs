use std::collections::HashSet;
use std::default::Default;
use std::fs::read_to_string;

pub fn main() {
    println!("Day 04");
    let input_file = "assets/day04/input.txt";
    let data = read_to_string(input_file).unwrap();
    let mut bingo = Bingo::default();
    bingo.parse_data(&data);
    println!("{:?}",bingo.boards[53]);
    //println!("> Read {} reports lines", parsed_data.len());
    let solution = solve_a(bingo);
    println!("> first winning score {}", solution);

    let mut bingo = Bingo::default();
    bingo.parse_data(&data);
    let solution = solve_b(bingo);
    println!("> last winning score {}", solution);
    //println!("> Allocations {}", r.allocations);
}

#[derive(Default, Debug)]
struct Bingo {
    extractions: Vec<u32>,
    current_extraction: usize,
    boards: Vec<Board>,
}

#[derive(Default, Debug)]
struct Board {
    numbers: [[u32;5]; 5],
    rows_and_columns: [HashSet<u32>; 10],
    total_sum: u32,
    extracted_sum: u32,
    exhausted: bool,
}

struct ExtractionResult<'a> {
    extracted_number :u32,
    winner_board: Option<&'a Board>,
}

impl Bingo {
    fn parse_data(&mut self, raw_data: &str) {
        let mut lines = raw_data.split('\n');
        self.extractions = lines.next().unwrap()
            .split(',')
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();
        lines.next();
        loop {
            let mut board = Board::default();
            for i in 0..5 {
                let line = lines.next();
                let mut col = 0;
                if let Some(l) = line {
                    l.split_whitespace()
                        .map(|s| s.trim().parse::<u32>().unwrap())
                        .for_each(|v| {
                            board.total_sum += v;
                            board.rows_and_columns[i].insert(v);
                            board.numbers[i][col] = v;
                            col += 1;
                    });
                } else {
                    break;
                }
            }
            if board.rows_and_columns[4].is_empty() {
                break;
            }
            lines.next();
            for r  in 0..5 {
                for c in 0..5 {
                    let val = board.numbers[r][c];
                    board.rows_and_columns[5+c].insert(val);
                    //println!("{}x{} {}",r,c, val);
                }
            }
            self.boards.push(board);
        }
    }

    fn extract_number(&mut self) -> ExtractionResult {
        let extracted_number = *self.extractions.get(self.current_extraction).unwrap();
        self.current_extraction += 1;


        let mut winner_board_idx = None;
        //println!("---> Extracted {}", extracted_number);
        for board_idx in 0..self.boards.len() {
            let board = self.boards.get_mut(board_idx).unwrap();
            if board.exhausted {
                continue;
            }
            let mut found = false;
            for i in 0..10 {
                if board.rows_and_columns[i].contains(&extracted_number) {
                    //println!("------> Found in board: {} line: {} remaining numbers: {}", board_idx + 1, i + 1, board.rows_and_columns[i].len());
                    //println!("------> {:?}", board.rows_and_columns[i]);
                    found = true;
                    board.rows_and_columns[i].remove(&extracted_number);

                }
                if board.rows_and_columns[i].is_empty() {
                    winner_board_idx = Some(board_idx);
                    board.exhausted = true;
                    break;
                }
            }
            if found {
                board.extracted_sum += extracted_number;
            }
        }

        //println!("> {}  {:?}", extracted_number, winner_board_idx);
        //println!("{:?}", self.boards.get(winner_board_idx.unwrap_or(1000)));

        ExtractionResult {
            extracted_number,
            winner_board: winner_board_idx.and_then(|idx| self.boards.get(idx)),
        }
    }
}

fn solve_a(bingo: Bingo) -> u32 {
    let mut bingo = bingo;
    loop {
        let extraction = bingo.extract_number();
        if let Some(board) = extraction.winner_board {
            return extraction.extracted_number * (board.total_sum - board.extracted_sum);
        }
    }
}


fn solve_b(bingo: Bingo) -> u32 {
    let mut bingo = bingo;
    let mut last_score= 0;
    for _ in 0..bingo.extractions.len() {
        let extraction = bingo.extract_number();
        if let Some(board) = extraction.winner_board {
            last_score = extraction.extracted_number * (board.total_sum - board.extracted_sum);
        }
    }
    last_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let data = read_to_string("assets/day04/input_sample.txt").unwrap();
        let mut bingo = Bingo::default();
        bingo.parse_data(&data);
        println!("{:?}", bingo);
        let solution = solve_a(bingo);
        assert_eq!(4512, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = read_to_string("assets/day04/input_sample.txt").unwrap();
        let mut bingo = Bingo::default();
        bingo.parse_data(&data);
        println!("{:?}", bingo);
        let solution = solve_b(bingo);
        assert_eq!(1924, solution);
    }

}
