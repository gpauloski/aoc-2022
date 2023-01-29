use std::cell::RefCell;
use std::rc::Rc;

use aoc;
use aoc::tree::Node;

enum FileType {
    DIR,
    FILE,
}

struct File {
    pub kind: FileType,
    pub name: String,
    pub size: u32,
}

impl File {
    fn new_dir(name: &str) -> File {
        File {
            kind: FileType::DIR,
            name: name.into(),
            size: 0,
        }
    }

    fn new_file(name: &str, size: u32) -> File {
        File {
            kind: FileType::FILE,
            name: name.into(),
            size: size,
        }
    }
}

fn get_child(node: Rc<RefCell<Node<File>>>, name: &str) -> Option<Rc<RefCell<Node<File>>>> {
    for child in node.children {
        if child.value.name == name {
            return Some(child);
        }
    }
    None
}

fn _format(node: Rc<RefCell<Node<File>>>, offset: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut s = " ".repeat(offset);
    let line = match node.value.kind {
        FileType::DIR => {
            format!("- {} (dir)", node.value.name)
        }
        FileType::FILE => {
            format!("- {} (file, size={})", node.value.name, node.value.size)
        }
    };
    s += line.as_str();
    result.push(s);
    for child in node.children {
        result.append(_format(child, offset + 2));
    }
    result
}

fn format(node: Rc<RefCell<Node<File>>>) -> String {
    _format(node, 0).join("\n")
}

fn parse(input: &str) -> Rc<RefCell<Node<File>>> {
    let mut lines = aoc::str_to_lines(input);

    if lines.pop().unwrap() != "$ cd /" {
        panic!("First command must be $ cd /");
    }
    let root = Node::new(File::new_dir("/"));
    let mut cwd = Rc::clone(&root);

    for line in lines {
        // Go up a directory
        if line == "$ cd .." {
            let cwd_clone = Rc::clone(&cwd);
            cwd = Rc::clone(cwd_clone.borrow().parent.as_ref().unwrap());
        // Enter directory
        } else if line.starts_with("$ cd ") {
            let (_, name) = line.rsplit_once(" ").unwrap();
            let cwd_clone = Rc::clone(&cwd);
            cwd = Rc::clone(cwd_clone.borrow().get(name).unwrap());
        // ls output directory (add if not known)
        } else if line.starts_with("dir") {
            let (_, name) = line.rsplit_once(" ").unwrap();
            match get_child(cwd, name) {
                Some(_) => {}
                None => {
                    let n = Node::new(File::new_dir(name));
                    cwd.borrow_mut().add_child(Rc::clone(&n));
                }
            }
        // ls output file (starts with file size, add if not known)
        } else if line.chars().next().unwrap().is_numeric() {
            let parts: Vec<&str> = line.split(" ").collect();
            let name = parts[1];
            let size = parts[0].parse::<u32>().unwrap();
            match get_child(cwd, name) {
                Some(_) => {}
                None => {
                    let n = Node::new(File::new_file(name, size));
                    cwd.borrow_mut().add_child(Rc::clone(&n));
                }
            }
        }
    }

    root
}

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

    static TEST_INPUT: &'static str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    static TEST_FORMATTED: &'static str = "\
- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)";

    #[test]
    fn test_parse() {
        let file = parse(TEST_INPUT);
        assert_eq!(format(file), TEST_FORMATTED);
    }

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
