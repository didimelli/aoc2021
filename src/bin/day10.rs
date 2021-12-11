fn main() {
    let input = parse_input("src/bin/day10_input.txt");
    println!("Result for part one is: {}", part_one(&input));
    println!("Result for part two is: {}", part_two(&input));
}

#[derive(Debug)]
struct Line {
    c: Vec<char>,
}

impl Line {
    fn match_opener_to_closer(opener: &char) -> char {
        match opener {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Unexpected opener!"),
        }
    }

    fn match_closer_to_point(closer: &char) -> u32 {
        match closer {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unexpected closer!"),
        }
    }

    fn match_closer_to_point_part_2(closer: &char) -> u32 {
        match closer {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected closer!"),
        }
    }

    fn inspect(&self) -> Option<char> {
        let mut opened: Vec<char> = vec![];
        let openers = vec!['(', '[', '{', '<'];
        let closers = vec![')', ']', '}', '>'];
        for c in self.c.iter() {
            if openers.contains(&c) {
                opened.push(*c);
            }
            if closers.contains(c) {
                // check if closes last opened. is it does, remove last from opened, else return closer
                if *c == Line::match_opener_to_closer(&opened.pop().unwrap()) {
                    continue;
                } else {
                    return Some(*c);
                }
            }
        }
        None
    }

    fn is_corrupted(&self) -> bool {
        self.inspect().is_some()
    }

    fn fix(&self) -> Vec<char> {
        // do something like self.inspect(). at the end of the iterations, opened should contain only
        // items that need closing. iter over them and match the closing chars!
        let mut fixers: Vec<char> = vec![];
        let mut opened: Vec<char> = vec![];
        let openers = vec!['(', '[', '{', '<'];
        let closers = vec![')', ']', '}', '>'];
        for c in self.c.iter() {
            if openers.contains(&c) {
                opened.push(*c);
            }
            if closers.contains(c) {
                // check if closes last opened. is it does, remove last from opened, else return closer
                if *c == Line::match_opener_to_closer(&opened.pop().unwrap()) {
                    continue;
                }
            }
        }
        while opened.len() > 0 {
            fixers.push(Line::match_opener_to_closer(&opened.pop().unwrap()))
        }
        while let Some(open) = opened.pop() {
            fixers.push(Line::match_opener_to_closer(&open));
        }
        fixers
    }
}

fn parse_input(filename: &str) -> Vec<Line> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Something wrong reading the file");
    content
        .lines()
        .map(|line| Line {
            c: line.chars().collect(),
        })
        .collect()
}

fn part_one(lines: &Vec<Line>) -> u32 {
    let mut error_score = 0;
    lines.iter().for_each(|line| {
        if let Some(closer) = line.inspect() {
            error_score += Line::match_closer_to_point(&closer);
        }
    });
    error_score
}

fn part_two(lines: &Vec<Line>) -> u64 {
    let mut scores: Vec<u64> = vec![];
    lines.iter().for_each(|line| {
        if !line.is_corrupted() {
            scores.push(line.fix().iter().fold(0, |acc, x| {
                acc * 5 + Line::match_closer_to_point_part_2(x) as u64
            }));
        }
    });
    scores.sort();
    scores[scores.len() / 2]
}

#[test]
fn part_one_works() {
    let input = parse_input("src/bin/day10_example.txt");
    assert_eq!(part_one(&input), 26397);
}

#[test]
fn part_two_works() {
    let input = parse_input("src/bin/day10_example.txt");
    assert_eq!(part_two(&input), 288957);
}
