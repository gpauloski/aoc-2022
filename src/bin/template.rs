use aoc;

fn part1(_input: &str) -> String {
    String::from("null")
}

fn part2(_input: &str) -> String {
    String::from("null")
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

    static TEST_INPUT: &'static str = "test input";

    #[test]
    fn test_part1() {
        let expected = String::from("null");
        let result = part1(TEST_INPUT);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let expected = String::from("null");
        let result = part2(TEST_INPUT);

        assert_eq!(result, expected);
    }
}
