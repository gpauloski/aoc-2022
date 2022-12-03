use aoc;

fn parse(input: &str) -> Vec<Vec<i32>> {
    let line_iter = input.split("\n").map(String::from);
    let mut groups: Vec<Vec<i32>> = Vec::new();

    let mut current: Vec<i32> = Vec::new();
    for raw_line in line_iter {
        let line = raw_line.trim();

        if line.is_empty() && current.len() > 0 {
            groups.push(current.clone());
            current.clear();
        } else if !line.is_empty() {
            let num: i32 = line.trim().parse().unwrap();
            current.push(num);
        }
    }

    if current.len() > 0 {
        groups.push(current);
    }

    groups
}

fn reduce(input: &Vec<Vec<i32>>) -> Vec<i32> {
    input.iter().map(|v| v.iter().sum()).collect()
}

fn compute_part1(input: &str) -> i32 {
    let groups = parse(input);
    let sums = reduce(&groups);

    let max = sums.iter().max();
    match max {
        Some(x) => *x,
        None => panic!("Found no groups of integers"),
    }
}

fn compute_part2(input: &str) -> i32 {
    let groups = parse(input);
    let mut sums = reduce(&groups);

    sums.sort_by(|a, b| b.cmp(a));

    if sums.len() < 3 {
        panic!("Found fewer than 3 groups");
    }

    sums[0..3].iter().sum()
}

fn main() {
    let config = aoc::read_cli_args();
    let input = aoc::read_input_file(&config.input_file);

    println!("Input file: {}", config.input_file);

    println!("Part 1 Answer: {}", compute_part1(&input));
    println!("Part 2 Answer: {}", compute_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = String::from(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        let mut result = compute_part1(&input);
        assert_eq!(result, 24000);

        result = compute_part2(&input);
        assert_eq!(result, 45000);
    }
}
