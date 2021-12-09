use std::{collections::HashMap, collections::HashSet};

fn main() {
    let lines = parse_input("src/bin/day8_input.txt");
    println!("Result for part one is: {}", part_one(&lines));
    println!("Result for part two is: {}", part_two(&lines));
}

#[derive(Debug)]
struct Line {
    left: Vec<Sequence>,
    right: Vec<Sequence>,
}

impl Line {
    fn decode(&self) -> u32 {
        // from left side i should get an hashmap <Sequence: u32>, to map sequence to the actual number? So that i can check right side against it.
        let mut map = HashMap::new();
        let one: HashSet<char> = self.item_to_set(&self.left, 2);
        map.insert(1, &one);
        let four: HashSet<char> = self.item_to_set(&self.left, 4);
        map.insert(4, &four);
        let seven: HashSet<char> = self.item_to_set(&self.left, 3);
        map.insert(7, &seven);
        let eight: HashSet<char> = self.item_to_set(&self.left, 7);
        map.insert(8, &eight);
        let sequences_of_six = self.items_of_len(6);
        let four_minus_one: HashSet<_> = four.difference(&one).map(|x| *x).collect();
        let zero = &sequences_of_six
            .iter()
            .filter(|x| {
                self.map_six_length_sequence(&eight, x, &four_minus_one, false)
                    .is_some()
            })
            .next()
            .unwrap();
        map.insert(0, *zero);
        let nine = &sequences_of_six
            .iter()
            .filter(|x| {
                self.map_six_length_sequence(&eight, x, &four, true)
                    .is_some()
            })
            .next()
            .unwrap();
        map.insert(9, *nine);
        let six = &sequences_of_six
            .iter()
            .filter(|x| x != nine && x != zero)
            .next()
            .unwrap();
        map.insert(6, *six);
        let sequences_of_five = self.items_of_len(5);
        let three = &sequences_of_five
            .iter()
            .filter(|x| one.is_subset(&x))
            .next()
            .unwrap();
        map.insert(3, *three);
        let five = &sequences_of_five
            .iter()
            .filter(|x| x.is_subset(&six))
            .next()
            .unwrap();
        map.insert(5, *five);
        let two = &sequences_of_five
            .iter()
            .filter(|x| x != three && x != five)
            .next()
            .unwrap();
        map.insert(2, *two);
        let mut sum: Vec<u32> = vec![];
        for s in &self.right {
            for (v, hash) in map.iter() {
                if **hash == s.set() {
                    sum.push(*v);
                }
            }
        }
        sum[0] * 1000 + sum[1] * 100 + sum[2] * 10 + sum[3]
    }

    fn map_six_length_sequence(
        &self,
        eight: &HashSet<char>,
        possible_zero: &HashSet<char>,
        superset: &HashSet<char>,
        rev: bool,
    ) -> Option<HashSet<char>> {
        let diff: HashSet<_> = eight.difference(possible_zero).map(|x| *x).collect();
        if rev {
            if !diff.is_subset(superset) {
                Some(possible_zero.clone())
            } else {
                None
            }
        } else {
            if diff.is_subset(superset) {
                Some(possible_zero.clone())
            } else {
                None
            }
        }
    }

    fn item_to_set(&self, sequences: &Vec<Sequence>, length: u32) -> HashSet<char> {
        sequences
            .iter()
            .filter(|x| x.s.len() == length as usize)
            .next()
            .unwrap()
            .set()
    }

    fn items_of_len(&self, length: u32) -> Vec<HashSet<char>> {
        self.left
            .iter()
            .filter(|s| s.s.len() == length as usize)
            .map(|s| s.set())
            .collect()
    }
}

#[derive(Debug)]
struct Sequence {
    s: String,
}

impl Sequence {
    fn is_unique(&self) -> bool {
        match self.s.len() {
            2 | 4 | 3 | 7 => true,
            _ => false,
        }
    }

    fn set(&self) -> HashSet<char> {
        let mut set = HashSet::new();
        self.s.chars().for_each(|c| {
            set.insert(c);
        });
        set
    }
}

fn part_one(lines: &Vec<Line>) -> u32 {
    let mut count = 0;
    for line in lines {
        line.right.iter().for_each(|s| {
            if s.is_unique() {
                count += 1;
            }
        });
    }
    count
}

fn part_two(lines: &Vec<Line>) -> u32 {
    lines.iter().fold(0, |acc, x| acc + x.decode())
}

fn parse_line(line: &str) -> Line {
    let mut split = line.split("|");
    let left = split.next().unwrap().trim().split(" ");
    let parsed_left: Vec<Sequence> = left.map(|seq| Sequence { s: seq.to_string() }).collect();
    let right = split.next().unwrap().trim().split(" ");
    let parsed_right: Vec<Sequence> = right.map(|seq| Sequence { s: seq.to_string() }).collect();
    Line {
        left: parsed_left,
        right: parsed_right,
    }
}

fn parse_input(filename: &str) -> Vec<Line> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Cannot read file");
    content.lines().map(|line| parse_line(line)).collect()
}

#[test]
fn part_one_works() {
    let lines = parse_input("src/bin/day8_example.txt");
    assert_eq!(part_one(&lines), 26);
}
#[test]
fn part_two_works() {
    let lines = parse_input("src/bin/day8_example.txt");
    assert_eq!(part_two(&lines), 61229);
}

#[test]
fn hashset() {
    let lines = parse_input("src/bin/day8_example.txt");
    for l in lines {
        l.decode();
    }
}
