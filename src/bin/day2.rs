use aoc;

#[derive(Clone, PartialEq)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(PartialEq)]
enum Result {
    WIN,
    DRAW,
    LOSS,
}

struct Round1 {
    them: Move,
    me: Move,
}

impl Round1 {
    pub fn new(them_code: char, me_code: char) -> Round1 {
        let them = match them_code {
            'A' => Move::ROCK,
            'B' => Move::PAPER,
            'C' => Move::SCISSORS,
            _ => panic!("Unknown code type: {}", them_code),
        };
        let me = match me_code {
            'X' => Move::ROCK,
            'Y' => Move::PAPER,
            'Z' => Move::SCISSORS,
            _ => panic!("Unknown code type: {}", me_code),
        };
        Round1 { them, me }
    }

    pub fn parse(lines: Vec<String>) -> Vec<Round1> {
        let mut rounds: Vec<Round1> = Vec::new();

        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            if chars.len() != 3 {
                panic!("Line '{}' is not length 3!", line);
            }
            rounds.push(Round1::new(chars[0], chars[2]));
        }

        rounds
    }

    pub fn result(&self) -> Result {
        if self.them == self.me {
            Result::DRAW
        } else if (self.me == Move::ROCK && self.them == Move::SCISSORS)
            || (self.me == Move::PAPER && self.them == Move::ROCK)
            || (self.me == Move::SCISSORS && self.them == Move::PAPER)
        {
            Result::WIN
        } else {
            Result::LOSS
        }
    }

    pub fn score(&self) -> u32 {
        let mut score: u32 = match self.me {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3,
        };

        score += match self.result() {
            Result::WIN => 6,
            Result::DRAW => 3,
            Result::LOSS => 0,
        };

        score
    }
}

struct Round2 {
    me: Move,
    result: Result,
}

impl Round2 {
    pub fn new(them_code: char, result_code: char) -> Round2 {
        let them = match them_code {
            'A' => Move::ROCK,
            'B' => Move::PAPER,
            'C' => Move::SCISSORS,
            _ => panic!("Unknown code type: {}", them_code),
        };
        let result = match result_code {
            'X' => Result::LOSS,
            'Y' => Result::DRAW,
            'Z' => Result::WIN,
            _ => panic!("Unknown code type: {}", result_code),
        };
        let me = Round2::get_move(&them, &result);
        Round2 { me, result }
    }

    pub fn parse(lines: Vec<String>) -> Vec<Round2> {
        let mut rounds: Vec<Round2> = Vec::new();

        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            if chars.len() != 3 {
                panic!("Line '{}' is not length 3!", line);
            }
            rounds.push(Round2::new(chars[0], chars[2]));
        }

        rounds
    }

    pub fn get_move(them: &Move, result: &Result) -> Move {
        match them {
            Move::ROCK => match result {
                Result::LOSS => Move::SCISSORS,
                Result::DRAW => Move::ROCK,
                Result::WIN => Move::PAPER,
            },
            Move::PAPER => match result {
                Result::LOSS => Move::ROCK,
                Result::DRAW => Move::PAPER,
                Result::WIN => Move::SCISSORS,
            },
            Move::SCISSORS => match result {
                Result::LOSS => Move::PAPER,
                Result::DRAW => Move::SCISSORS,
                Result::WIN => Move::ROCK,
            },
        }
    }

    pub fn score(&self) -> u32 {
        let mut score: u32 = match self.me {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3,
        };

        score += match self.result {
            Result::WIN => 6,
            Result::DRAW => 3,
            Result::LOSS => 0,
        };

        score
    }
}

fn compute_part1(input: &str) -> u32 {
    let lines = aoc::str_to_lines(input);
    let rounds = Round1::parse(lines);

    rounds.iter().map(Round1::score).sum()
}

fn compute_part2(input: &str) -> u32 {
    let lines = aoc::str_to_lines(input);
    let rounds = Round2::parse(lines);

    rounds.iter().map(Round2::score).sum()
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
    fn test() {
        let input = String::from("A Y\nB X\nC Z");

        assert_eq!(compute_part1(&input), 15);
        assert_eq!(compute_part2(&input), 12);
    }
}
