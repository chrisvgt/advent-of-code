use array2d::Array2D;
use std::string::*;

use super::super::utils::dbg::*;

fn is_accessible(array2d: &Array2D<char>, row: usize, column: usize) -> bool {
    let mut count = 0;

    let rows = array2d.row_len();
    let columns = array2d.column_len();

    let has_top = row > 0;
    let has_left = column > 0;

    let has_bottom = row + 1 < rows;
    let has_right = column + 1 < columns;

    if has_left && array2d.get(row, column - 1) == Some(&'@') {
        count += 1;
    }

    if has_right && array2d.get(row, column + 1) == Some(&'@') {
        count += 1;
    }

    if has_top && array2d.get(row - 1, column) == Some(&'@') {
        count += 1;
    }

    if has_bottom && array2d.get(row + 1, column) == Some(&'@') {
        count += 1;
    }

    if has_top && has_left && array2d.get(row - 1, column - 1) == Some(&'@') {
        count += 1;
    }

    if has_top && has_right && array2d.get(row - 1, column + 1) == Some(&'@') {
        count += 1;
    }

    if has_bottom && has_left && array2d.get(row + 1, column - 1) == Some(&'@') {
        count += 1;
    }

    if has_bottom && has_right && array2d.get(row + 1, column + 1) == Some(&'@') {
        count += 1;
    }
    count < 4
}

pub fn part1(input: &[String]) -> u64 {
    let rows = input.len();
    let columns = input[0].len();

    if rows != columns {
        return 0;
    }

    let data: Vec<char> = input.iter().flat_map(|s| s.chars()).collect();

    let array2d =
        Array2D::from_column_major(&data, rows, columns).expect("failed to create array2D");

    let mut count = 0;

    for i in 0..columns {
        for j in 0..rows {
            if array2d.get(j, i) == Some(&'@') {
                if is_accessible(&array2d, j, i) {
                    count += 1;
                    dbgprint!("X");
                } else {
                    dbgprint!("@");
                }
            } else {
                dbgprint!(".");
            }
        }
        dbgprintln!();
    }
    count
}

pub fn part2(input: &[String]) -> u64 {
    let rows = input.len();
    let columns = input[0].len();

    if rows != columns {
        return 0;
    }

    let data: Vec<char> = input.iter().flat_map(|s| s.chars()).collect();

    let mut array2d =
        Array2D::from_column_major(&data, rows, columns).expect("failed to create array2D");

    let mut sum = 0;
    let mut count = 1;

    while count > 0 {
        count = 0;
        for i in 0..columns {
            for j in 0..rows {
                if array2d.get(j, i) == Some(&'@') {
                    if is_accessible(&array2d, j, i) {
                        let _ = array2d.set(j, i, 'X');
                        count += 1;
                        dbgprint!("X");
                    } else {
                        dbgprint!("@");
                    }
                } else {
                    dbgprint!(".");
                }
            }
            dbgprintln!();
        }
        dbgprintln!();
        dbgprintln!();
        sum += count;
    }
    sum
}

#[allow(unused)]
pub fn solve(input: &[String]) {
    let now = std::time::Instant::now();
    let part1 = part1(input);
    let part2 = part2(input);
    let elapsed = now.elapsed();

    println!("--- Day 4: Printing Department ---");
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    println!("Elapsed: {:.2?}", elapsed);
}

#[test]
fn test1() {
    let v = vec![
        "..@@.@@@@.".to_string(),
        "@@@.@.@.@@".to_string(),
        "@@@@@.@.@@".to_string(),
        "@.@@@@..@.".to_string(),
        "@@.@@@@.@@".to_string(),
        ".@@@@@@@.@".to_string(),
        ".@.@.@.@@@".to_string(),
        "@.@@@.@@@@".to_string(),
        ".@@@@@@@@.".to_string(),
        "@.@.@@@.@.".to_string(),
    ];
    assert_eq!(part1(&v), 13);
}

#[test]
fn test2() {
    let v = vec![
        "..@@.@@@@.".to_string(),
        "@@@.@.@.@@".to_string(),
        "@@@@@.@.@@".to_string(),
        "@.@@@@..@.".to_string(),
        "@@.@@@@.@@".to_string(),
        ".@@@@@@@.@".to_string(),
        ".@.@.@.@@@".to_string(),
        "@.@@@.@@@@".to_string(),
        ".@@@@@@@@.".to_string(),
        "@.@.@@@.@.".to_string(),
    ];
    assert_eq!(part2(&v), 43);
}
