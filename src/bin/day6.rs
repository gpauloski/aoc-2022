use std::collections::HashSet;

use aoc;

// https://stackoverflow.com/questions/46766560
fn has_unique_elements(input: &str) -> bool {
    let mut uniq = HashSet::new();
    input.chars().all(move |x| uniq.insert(x))
}

fn find_marker(input: &str, count: usize) -> usize {
    for i in 0..(input.len() - count) {
        if has_unique_elements(&input[i..(i+count)]) {
            return i + count;
        }
    }
    0
}


fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn main() {
    let config = aoc::read_cli_args();
    let input = aoc::read_input_file(&config.input_file);

    println!("Input file: {}", config.input_file);
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    static TEST_INPUT_2: &'static str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    static TEST_INPUT_3: &'static str = "nppdvjthqldpwncqszvftbrmjlhg";
    static TEST_INPUT_4: &'static str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    static TEST_INPUT_5: &'static str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 7);
        assert_eq!(part1(TEST_INPUT_2), 5);
        assert_eq!(part1(TEST_INPUT_3), 6);
        assert_eq!(part1(TEST_INPUT_4), 10);
        assert_eq!(part1(TEST_INPUT_5), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 19);
        assert_eq!(part2(TEST_INPUT_2), 23);
        assert_eq!(part2(TEST_INPUT_3), 23);
        assert_eq!(part2(TEST_INPUT_4), 29);
        assert_eq!(part2(TEST_INPUT_5), 26);
    }
}
