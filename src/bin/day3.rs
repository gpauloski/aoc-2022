use aoc;

#[derive(Debug)]
struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
}

impl Rucksack {
    fn new(items: &String) -> Rucksack {
        if items.len() % 2 != 0 {
            panic!("Input must have even number of items: {}", items);
        }

        let left = items[..items.len() / 2].chars().collect();
        let right = items[items.len() / 2..].chars().collect();

        Rucksack { left, right }
    }

    fn misorganized(&self) -> Vec<char> {
        let mut x: Vec<char> = Vec::new();
        for c in &self.left {
            if self.right.contains(&c) && !x.contains(&c) {
                x.push(*c)
            }
        }
        x
    }

    fn all(&self) -> Vec<char> {
        let mut out = self.left.clone();
        out.extend(&self.right);
        out
    }
}

fn score(items: &Vec<char>) -> u32 {
    let mut score: u32 = 0;
    for c in items {
        let x = *c as u32;
        if x < 96 {
            score += x - 38;
        } else {
            score += x - 96;
        }
    }
    score
}

fn badge(r1: &Rucksack, r2: &Rucksack, r3: &Rucksack) -> char {
    let w1 = r1.all();
    let w2 = r2.all();
    let w3 = r3.all();

    for c in &w1 {
        if w2.contains(&c) && w3.contains(&c) {
            return *c;
        }
    }

    panic!("No badge found in {:?}, {:?}, {:?}", w1, w2, w3);
}

fn part1(input: &str) -> u32 {
    let items_list = aoc::str_to_lines(input);

    let rucksacks: Vec<Rucksack> = items_list.iter().map(Rucksack::new).collect();
    let wrong: Vec<Vec<char>> = rucksacks.iter().map(Rucksack::misorganized).collect();

    let scores: Vec<u32> = wrong.iter().map(score).collect();
    scores.iter().sum()
}

fn part2(input: &str) -> u32 {
    let items_list = aoc::str_to_lines(input);

    let rucksacks: Vec<Rucksack> = items_list.iter().map(Rucksack::new).collect();

    let mut badges: Vec<char> = Vec::new();
    for i in (0..rucksacks.len()).step_by(3) {
        badges.push(badge(&rucksacks[i], &rucksacks[i + 1], &rucksacks[i + 2]));
    }

    score(&badges)
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

    static TEST_INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let expected = 157;
        let result = part1(TEST_INPUT);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 70;
        let result = part2(TEST_INPUT);

        assert_eq!(result, expected);
    }
}
