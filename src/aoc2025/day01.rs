use std::string::*;

pub fn part1(input: &Vec<String>) -> usize {
    let mut position: usize = 50;
    let mut ret: usize = 0;

    for line in input {
        let dir = line.chars().next().unwrap();
        let number = line[1..].parse::<usize>().unwrap();
        let amount = number % 100;

        if dir == 'L' {
            if position < amount {
                position = 100 - (amount - position);
            } else {
                position -= amount;
            }
        } else if dir == 'R' {
            position = (position + amount) % 100;
        }

        if position == 0 {
            ret += 1;
        }
    }
    ret
}

pub fn part2(input: &Vec<String>) -> usize {
    let mut position = 50;
    let mut ret = 0;

    for line in input {
        let dir = line.chars().next().unwrap();
        let number = line[1..].parse::<usize>().unwrap();

        let amount = number % 100;
        let rot = number / 100;

        if rot > 0 {
            ret += rot;
        }
        let mut was = position == 0;

        if dir == 'L' {
            if position < amount {
                position = 100 - (amount - position);
                if !was {
                    ret += 1;
                    was = true;
                };
            } else {
                position -= amount;
            }
        } else if dir == 'R' {
            if position + amount >= 100 {
                position = (position + amount) % 100;
                if !was {
                    ret += 1;
                    was = true;
                };
            } else {
                position += amount;
            }
        }
        if position == 0 && !was {
            ret += 1;
        }
    }
    ret
}

#[allow(unused)]
pub fn solve(input: &Vec<String>) {
    let now = std::time::Instant::now();
    let part1 = part1(input);
    let part2 = part2(input);
    let elapsed = now.elapsed();

    println!("--- Day 1: Secret Entrance ---");
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}

#[test]
fn test1() {
    let v = vec![
        "L68".to_string(),
        "L30".to_string(),
        "R48".to_string(),
        "L5".to_string(),
        "R60".to_string(),
        "L55".to_string(),
        "L1".to_string(),
        "L99".to_string(),
        "R14".to_string(),
        "L82".to_string(),
    ];
    assert_eq!(part1(&v), 3);
}

#[test]
fn test2() {
    let v = vec![
        "L68".to_string(),
        "L30".to_string(),
        "R48".to_string(),
        "L5".to_string(),
        "R60".to_string(),
        "L55".to_string(),
        "L1".to_string(),
        "L99".to_string(),
        "R14".to_string(),
        "L82".to_string(),
    ];
    assert_eq!(part2(&v), 6);
}

#[test]
fn test3() {
    let v = vec!["R1000".to_string()];
    assert_eq!(part2(&v), 10);
}
