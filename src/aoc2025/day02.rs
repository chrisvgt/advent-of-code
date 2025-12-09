use std::string::*;

fn check1(str: &str) -> u64 {
    if str.starts_with('0') {
        return 0;
    }

    if !str.len().is_multiple_of(2) {
        return 0;
    }

    let slen = str.len() / 2;
    for i in 0..slen {
        if str.chars().nth(i).unwrap() != str.chars().nth(slen + i).unwrap() {
            return 0;
        }
    }
    str.parse().unwrap()
}

pub fn part1(input: &str) -> u64 {
    let mut ret: u64 = 0;
    let v: Vec<&str> = input.split_terminator(',').collect();

    for item in v {
        let range: Vec<&str> = item.split_terminator('-').collect();
        let start = range[0].parse::<u64>().unwrap();
        let end = range[1].parse::<u64>().unwrap();

        ret += (start..=end)
            .map(|i| check1(&i.to_string()[..]))
            .sum::<u64>();
    }
    ret
}

fn check2(str: &str) -> u64 {
    let slen: usize = str.len();

    for i in 1..=slen / 2 {
        let prefix = &str[..i].repeat(slen / i);
        if prefix == str {
            return str.parse().unwrap();
        }
    }
    0
}

pub fn part2(input: &str) -> u64 {
    let mut ret: u64 = 0;
    let v: Vec<&str> = input.split_terminator(',').collect();

    for item in v {
        let range: Vec<&str> = item.split_terminator('-').collect();
        let start = range[0].parse::<u64>().unwrap();
        let end = range[1].parse::<u64>().unwrap();

        ret += (start..=end)
            .map(|i| check2(&i.to_string()[..]))
            .sum::<u64>();
    }
    ret
}

#[allow(unused)]
pub fn solve(input: &str) {
    let now = std::time::Instant::now();
    let part1 = part1(input);
    let part2 = part2(input);
    let elapsed = now.elapsed();

    println!("--- Day 2: Gift Shop ---");
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}

#[test]
fn test1() {
    let str: String = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
    assert_eq!(part1(&str), 1227775554);
}

#[test]
fn test2() {
    let str: String = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
    assert_eq!(part2(&str), 4174379265);
}
