const INPUT: &str = include_str!("../../inputs/day02.txt");

fn solve(input: &str) -> i64 {
    input
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .flat_map(|range| {
            let mut parts = range.split('-').map(|n| n.trim().parse::<i64>().unwrap());
            let start = parts.next().unwrap();
            let end = parts.next().unwrap();
            start..=end
        })
        .filter(|&i| {
            let s = i.to_string();
            s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..]
        })
        .sum()
}

fn main() {
    println!("Result: {}", solve(INPUT));
}
