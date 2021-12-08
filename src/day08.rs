use std::collections::HashMap;
use std::fs::read_to_string;

pub fn main() {
    println!("Day 08");
    let input_file = "assets/day08/input.txt";
    let data = read_to_string(input_file).unwrap();
    let data: Vec<&str> = data.split('\n').collect();
    let solution = solve_a(&data);
    println!("> Known digits {}", solution);
    let solution = solve_b(&data);
    println!("> Sum outputs {}", solution);
}

fn solve_a(data: &[&str]) -> usize {
    data.iter()
        .map(|s| s.split('|').nth(1).unwrap())
        .map(|s| s.split_whitespace())
        .map(|s| s.filter(|code| matches!(code.len(), 2 | 3 | 4 | 7)).count())
        .sum()
}

fn count_commons<'a>(str1: &'a str, str2: &'a str) -> usize {
    str1.chars()
        .into_iter()
        .filter(|c| str2.contains(*c))
        .count()
}

fn solve_b(data: &[&str]) -> u32 {
    #[derive(Default)]
    struct WordBook<'a> {
        translations: HashMap<&'a str, u32>,
    }
    data.iter()
        .map(|s| {
            let words: Vec<_> = s.split(&['|', ' '][..]).collect();
            let count = words.len();
            let word_book =
                words
                    .iter()
                    .cycle()
                    .take(count * 2)
                    .fold(WordBook::default(), |mut wb, w| {
                        match w.len() {
                            2 => {
                                wb.translations.insert(w, 1);
                            }
                            3 => {
                                wb.translations.insert(w, 7);
                            }
                            4 => {
                                wb.translations.insert(w, 4);
                            }
                            7 => {
                                wb.translations.insert(w, 8);
                            }
                            5 => {
                                if let Some(word_4) = wb.translations.iter().find(|e| *e.1 == 4) {
                                    let len4 = count_commons(word_4.0, w);
                                    match len4 {
                                        2 => {
                                            wb.translations.insert(w, 2);
                                        }
                                        3 => {
                                            wb.translations.insert(w, 5);
                                        }
                                        _ => {}
                                    }
                                }
                                if let Some(word_1) = wb.translations.iter().find(|e| *e.1 == 1) {
                                    if 2 == count_commons(word_1.0, w) {
                                        wb.translations.insert(w, 3);
                                    };
                                }
                            }
                            6 => {
                                if let (Some(word_1), Some(word_4)) = (
                                    wb.translations.iter().find(|e| *e.1 == 1),
                                    wb.translations.iter().find(|e| *e.1 == 4),
                                ) {
                                    let len4 = count_commons(word_4.0, w);
                                    let len1 = count_commons(word_1.0, w);
                                    if len4 == 4 {
                                        wb.translations.insert(w, 9);
                                    } else if len1 == 2 {
                                        wb.translations.insert(w, 0);
                                    } else {
                                        wb.translations.insert(w, 6);
                                    }
                                }
                            }
                            _ => {}
                        }
                        wb
                    });

            // println!("> {:?}", word_book.translations);
            s.split('|')
                .skip(1)
                .into_iter()
                .map(|o| {
                    o.split_whitespace().fold(0, |mut acc, w| {
                        acc = acc * 10 + word_book.translations.get(w).unwrap();
                        acc
                    })
                })
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let data = read_to_string("assets/day08/input_sample.txt").unwrap();

        let data: Vec<&str> = data.split('\n').collect();
        let solution = solve_a(&data);
        assert_eq!(26, solution);
    }

    #[test]
    fn test_solve_b() {
        let data = read_to_string("assets/day08/input_sample.txt").unwrap();

        let data: Vec<&str> = data.split('\n').collect();
        let solution = solve_b(&data);
        assert_eq!(61229, solution);
    }
}
