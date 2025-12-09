mod aoc2025;
mod utils;

fn main() {
    /* Advent of Code 2025 */
    aoc2025::day01::solve(&utils::io::read_lines("./input/aoc2025/day01.txt"));
    aoc2025::day02::solve(&utils::io::read_string("./input/aoc2025/day02.txt"));
}
