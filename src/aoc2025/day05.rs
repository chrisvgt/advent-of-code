use super::super::utils::dbg::*;

fn is_fresh(item: u64, ranges: &Vec<Vec<u64>>) -> bool {
    for range in ranges {
        if item >= *range.first().unwrap() && item <= *range.last().unwrap() {
            return true;
        }
    }
    false
}

pub fn part1(input: &[String]) -> u64 {
    let mut ret = 0;
    let mut ranges: Vec<Vec<u64>> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    for line in input {
        let tmp = line
            .split_terminator('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if tmp.len() == 2 {
            ranges.push(tmp);
        } else if tmp.len() == 1 {
            ingredients.extend(tmp);
        }
    }

    for item in ingredients {
        if is_fresh(item, &ranges) {
            ret += 1;
        }
    }
    ret
}

pub fn part2(input: &[String]) -> u64 {
    let mut ranges: Vec<Vec<u64>> = Vec::new();

    for line in input {
        let tmp = line
            .split_terminator('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if tmp.len() == 2 {
            ranges.push(tmp);
        }
    }
    ranges.sort();
    dbgprintln!("{:?}", ranges);
    dbgprintln!();

    let mut last_end = 0;
    let mut sum = 0;

    for range in ranges {
        dbgprintln!("[{}-{}]", range.first().unwrap(), range.get(1).unwrap());
        let mut start = *range.first().unwrap();
        let end = *range.get(1).unwrap();
        let mut add = 0;

        if start < last_end {
            start = last_end;
        }

        if start == end && last_end == 0 {
            add += 1;
        } else if start < end {
            if last_end == start {
                add += end - start;
            } else {
                add += end + 1 - start;
            }
        }

        dbgprintln!(
            "sum:{} start:{} end:{} last_end:{}",
            sum,
            start,
            end,
            last_end
        );
        dbgprintln!("{} --> {}", start, range.get(1).unwrap());
        dbgprintln!("sum: {} + {} = {}", sum, add, sum + add);

        if last_end < end {
            last_end = end;
        }
        sum += add;

        dbgprintln!("---");
    }
    sum
}

#[allow(unused)]
pub fn solve(input: &[String]) {
    let now = std::time::Instant::now();
    let part1 = part1(input);
    let part2 = part2(input);
    let elapsed = now.elapsed();

    println!("--- Day 5: Cafeteria ---");
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}

#[test]
fn test1() {
    let v = vec![
        "3-5".to_string(),
        "10-14".to_string(),
        "16-20".to_string(),
        "12-18".to_string(),
        "".to_string(),
        "1".to_string(),
        "5".to_string(),
        "8".to_string(),
        "11".to_string(),
        "17".to_string(),
        "32".to_string(),
    ];
    assert_eq!(part1(&v), 3);
}

#[test]
fn test2() {
    let v = vec![
        "3-5".to_string(),
        "10-14".to_string(),
        "16-20".to_string(),
        "12-18".to_string(),
        "".to_string(),
        "1".to_string(),
        "5".to_string(),
        "8".to_string(),
        "11".to_string(),
        "17".to_string(),
        "32".to_string(),
    ];
    assert_eq!(part2(&v), 14);
}

#[test]
fn test3() {
    let v = vec![
        "3-5".to_string(),
        "10-14".to_string(),
        "16-20".to_string(),
        "12-18".to_string(),
        "3-3".to_string(),
        "17-20".to_string(),
        "11-11".to_string(),
        "".to_string(),
        "1".to_string(),
        "5".to_string(),
        "8".to_string(),
        "11".to_string(),
        "17".to_string(),
        "32".to_string(),
    ];
    assert_eq!(part2(&v), 14);
}
