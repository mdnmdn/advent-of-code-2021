use std::fs::read_to_string;
use std::ops::Add;
use std::slice::Iter;

pub fn main() {
    println!("Day 01");
    let input_file = "assets/day01/input.txt";
    let data = read_to_string(input_file).unwrap();
    //println!("{}", data);
    let parsed_data = parse_file(data);
    //println!("{:?}",parsed_data);
    println!("> Read {} sweeps", parsed_data.len());
    let solution = solve_a(&parsed_data);
    println!("> Found {} descending sweeps", solution);
    let solution = solve_b(&parsed_data);
    println!("> Found {} descending sweeps with mobile average", solution);
    let solution = solve_b_with_sliding_iterator(&parsed_data);
    println!(
        "> Found {} descending sweeps with sliding window iterator",
        solution
    );
}

fn parse_file(raw_data: String) -> Vec<i32> {
    raw_data
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn solve_a(data: &[i32]) -> usize {
    let mut ahead_iter = data.iter();
    ahead_iter.next();
    ahead_iter.zip(data.iter()).filter(|i| i.0 > i.1).count()
}

fn solve_b(data: &[i32]) -> usize {
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
        acc.position += 1;
        let sum = acc.sliding_values[0] + acc.sliding_values[1] + acc.sliding_values[2];
        if acc.position > 3 && sum > acc.previous_value {
            acc.sweeps += 1;
        }
        acc.previous_value = sum;

        acc
    });
    result.sweeps
}

// ------------- WITH SLIDING ITERATOR -----------------

fn solve_b_with_sliding_iterator(data: &[i32]) -> usize {
    let mut ahead_iter = SlidingIterator::new(data);
    ahead_iter.next();
    ahead_iter
        .zip(SlidingIterator::new(data))
        .filter(|i| i.0 > i.1)
        .count()
}

struct SlidingIterator<'a, T>
where
    T: Add + Copy,
{
    base_iterator: Iter<'a, T>,
    sliding_values: [Option<&'a T>; 3],
    previous_value: Option<T>,
    position: usize,
}

impl<'a, T> SlidingIterator<'a, T>
where
    T: Add + Copy,
{
    fn new(data: &'a [T]) -> Self {
        let mut base_iterator = data.iter();
        let sliding_values = [base_iterator.next(), base_iterator.next(), None];

        SlidingIterator {
            base_iterator,
            sliding_values,
            previous_value: None,
            position: 1,
        }
    }
}

impl<'a, T> Iterator for SlidingIterator<'a, T>
where
    T: Add + Copy + Default,
    T: Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.base_iterator.next() {
            Some(v) => {
                self.position += 1;
                self.sliding_values[self.position % 3] = Some(v);
                let sum = self.sliding_values[0]
                    .unwrap()
                    .add(*self.sliding_values[1].unwrap())
                    .add(*self.sliding_values[2].unwrap());

                self.previous_value = Some(sum);
                Some(sum)
            }
            _ => None,
        }
    }
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

    #[test]
    fn test_solve_b_sliding_iterator() {
        let data = get_test_data();
        let solution = solve_b_with_sliding_iterator(&data);
        assert_eq!(5, solution);
    }
}
