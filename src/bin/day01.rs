const INPUT: &str = include_str!("../../inputs/day01.txt");

fn part1(input: &str) -> i64 {
    let mut click_count = 0;
    let mut current = 50;
    for line in input.lines() {
        let first_char = line.chars().next().unwrap();
        let mut number = line[1..].parse::<i64>().unwrap();
        let delta = match first_char {
            'R' => number,    
            'L' => -number,       
            _ => continue
        };

        current = (current + delta).rem_euclid(100);
        if current == 0 {
            click_count += 1;
        }
    }

    click_count
}

fn main() {
    println!("Result: {}", part1(INPUT));
}
