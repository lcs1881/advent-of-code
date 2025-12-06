const INPUT: &str = include_str!("../../inputs/day03.txt");

fn solve(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
            let (tens_pos, &tens_digit) = digits[..digits.len() - 1]
                .iter()
                .enumerate()
                .max_by_key(|(_, d)| **d)
                .unwrap();
            
            let ones_digit = digits[tens_pos + 1..]
                .iter()
                .max()
                .unwrap();
            
            tens_digit as i64 * 10 + *ones_digit as i64
        })
        .sum()
}

fn main() {
    println!("Result: {}", solve(INPUT));
}
