use aoc;

fn compute(input: String) -> String {
    String::from("null")
}

fn main() {
    let config = aoc::read_cli_args();
    let input = aoc::read_input_file(&config.input_file);

    println!("Input file: {}", config.input_file);
    let result = compute(input);
    println!("Answer: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = String::from("test input");
        let expected = String::from("null");

        let result = compute(input);

        assert_eq!(result, expected);
    }
}
