use aoc;

#[derive(PartialEq)]
enum MoverType {
    V9000,
    V9001,
}

#[derive(Debug)]
struct Move {
    count: u32,
    source: u32,
    dest: u32,
}

impl Move {
    fn parse(move_: &str) -> Move {
        let tokens: Vec<&str> = move_.split(' ').collect();
        let count = tokens[1].parse::<u32>().unwrap();
        let source: u32 = tokens[3].parse::<u32>().unwrap() - 1;
        let dest: u32 = tokens[5].parse::<u32>().unwrap() - 1;
        Move {
            count,
            source,
            dest,
        }
    }

    fn parse_lines(moves: Vec<String>) -> Vec<Move> {
        moves.iter().map(|m| Move::parse(m.as_str())).collect()
    }
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn parse(mut lines: Vec<String>) -> Stacks {
        let num_line = lines.pop().unwrap();
        let mut nums_parts: Vec<&str> = num_line.split(' ').collect();
        nums_parts.retain(|&s| s.len() != 0);
        let nums: Vec<u32> = nums_parts
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let mut stacks: Vec<Vec<char>> = Vec::new();
        let size = nums.iter().max().unwrap().clone();
        for _ in 0..size {
            stacks.push(Vec::new());
        }

        for line in lines.iter().rev() {
            for (i, c) in line.chars().enumerate() {
                if i % 4 == 1 && c != ' ' {
                    stacks[(i - 1) / 4].push(c);
                }
            }
        }

        Stacks { stacks }
    }

    fn apply_move(&mut self, move_: &Move, version: &MoverType) {
        match version {
            MoverType::V9000 => {
                for _ in 0..move_.count {
                    let c = self.stacks[move_.source as usize].pop().unwrap();
                    self.stacks[move_.dest as usize].push(c);
                }
            }
            MoverType::V9001 => {
                let stack = &mut self.stacks[move_.source as usize];
                let split_index = stack.len() - (move_.count as usize);
                let mut substack = stack.split_off(split_index);
                self.stacks[move_.dest as usize].append(&mut substack);
            }
        }
    }

    fn apply_moves(&mut self, moves: &Vec<Move>, version: MoverType) {
        for move_ in moves {
            self.apply_move(move_, &version);
        }
    }

    fn max_height(&self) -> usize {
        match self.stacks.iter().map(Vec::len).max() {
            Some(x) => x,
            None => 0,
        }
    }

    fn show(&self) -> String {
        let mut s = String::new();
        for row in (0..self.max_height()).rev() {
            let mut line = String::new();
            for stack in self.stacks.iter() {
                if stack.len() > 0 && row <= stack.len() - 1 {
                    line += format!("[{}]", stack[row]).as_str();
                } else {
                    line += "   ";
                }
            }
            line += "\n";
            s += line.as_str();
        }
        s
    }

    fn tops(&self) -> String {
        self.stacks.iter().map(|v| v.last().unwrap()).collect()
    }
}

fn parse(input: &str) -> (Stacks, Vec<Move>) {
    let mut stacks_ = aoc::str_to_lines(input);
    let empty_line = stacks_.iter().position(|x| x.len() == 0).unwrap();
    let mut moves_ = stacks_.split_off(empty_line);
    moves_.retain(|m| m.len() > 0);

    let stacks = Stacks::parse(stacks_);
    let moves = Move::parse_lines(moves_);

    (stacks, moves)
}

fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse(input);

    println!("Starting stacks:\n{}", stacks.show());
    stacks.apply_moves(&moves, MoverType::V9000);
    println!("Final stacks:\n{}", stacks.show());
    stacks.tops()
}

fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse(input);

    println!("Starting stacks:\n{}", stacks.show());
    stacks.apply_moves(&moves, MoverType::V9001);
    println!("Final stacks:\n{}", stacks.show());
    stacks.tops()
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

    static TEST_INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let expected = String::from("CMZ");
        let result = part1(TEST_INPUT);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let expected = String::from("MCD");
        let result = part2(TEST_INPUT);

        assert_eq!(result, expected);
    }
}
