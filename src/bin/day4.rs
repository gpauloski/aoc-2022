use aoc;

#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn new(section: &str) -> Section {
        let nums: Vec<&str> = section.split('-').collect();
        if nums.len() != 2 {
            panic!(
                "Section string must contain two numbers separated \
                   by a dash. E.g., 5-24"
            );
        }
        let start = nums[0].parse::<u32>().unwrap();
        let end = nums[1].parse::<u32>().unwrap();
        Section { start, end }
    }

    fn contains(&self, other: &Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn disjoint(&self, other: &Section) -> bool {
        (self.start < other.start && self.end < other.start)
            || (self.start > other.end && self.end > other.end)
    }
}

fn parse(input: &str) -> Vec<(Section, Section)> {
    let lines = aoc::str_to_lines(input);
    let mut section_pairs: Vec<(Section, Section)> = Vec::new();
    for line in lines {
        let s: Vec<&str> = line.split(',').collect();
        let left = s[0];
        let right = s[1];
        section_pairs.push((Section::new(left), Section::new(right)));
    }
    section_pairs
}

fn part1(input: &str) -> u32 {
    let sections = parse(input);

    let mut contained: u32 = 0;
    for (s1, s2) in sections {
        if s1.contains(&s2) || s2.contains(&s1) {
            contained += 1;
        }
    }

    contained
}

fn part2(input: &str) -> u32 {
    let sections = parse(input);

    let mut overlapped: u32 = 0;
    for (s1, s2) in sections {
        if !s1.disjoint(&s2) {
            overlapped += 1;
        }
    }

    overlapped
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

    static TEST_INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let expected = 2;
        let result = part1(TEST_INPUT);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 4;
        let result = part2(TEST_INPUT);

        assert_eq!(result, expected);
    }
}
