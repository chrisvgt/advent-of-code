use std::string::*;

pub fn part1(input: &Vec<String>) -> u64 {
    let mut ret: u64 = 0;
    for item in input {
        let digits = item
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();

        let max = digits[..digits.len() - 1].iter().max().unwrap();
        let pos_max = digits[..digits.len() - 1]
            .iter()
            .position(|x| x == max)
            .unwrap();
        let next_max = digits[pos_max + 1..digits.len()].iter().max().unwrap();
        ret += (*max as u64 * 10) + *next_max as u64;
    }
    ret
}

#[allow(unused)]
pub fn part2(input: &Vec<String>) -> u64 {
    0
}

#[allow(unused)]
pub fn solve(input: &Vec<String>) {
    let now = std::time::Instant::now();
    let part1 = part1(input);
    let part2 = part2(input);
    let elapsed = now.elapsed();

    println!("--- Day 3: Lobby ---");
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}

#[test]
fn test1() {
    let v = vec![
        "987654321111111".to_string(),
        "811111111111119".to_string(),
        "234234234234278".to_string(),
        "818181911112111".to_string(),
    ];
    assert_eq!(part1(&v), 357);
}

#[test]
fn test2() {
    let v = vec!["818181911119111".to_string()];
    assert_eq!(part1(&v), 99);
}

#[test]
fn test3() {
    let v = vec![
        "987654321111111".to_string(),
        "811111111111119".to_string(),
        "234234234234278".to_string(),
        "818181911112111".to_string(),
    ];
    assert_eq!(part2(&v), 3121910778619);
}
