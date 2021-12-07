use std::collections::HashMap;

fn main() {
    let lines = parse_input("src/bin/day5_input.txt");
    println!("Result for part one is: {}", part_one(&lines));
    println!("Result for part two is: {}", part_two(&lines));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn line_or_column(&self) -> Vec<Point> {
        if self.p1.x == self.p2.x {
            // column
            if self.p2.y > self.p1.y {
                return (self.p1.y..self.p2.y + 1)
                    .map(|y_coord| Point {
                        x: self.p1.x,
                        y: y_coord,
                    })
                    .collect();
            } else {
                return (self.p2.y..self.p1.y + 1)
                    .map(|y_coord| Point {
                        x: self.p1.x,
                        y: y_coord,
                    })
                    .collect();
            }
        } else if self.p1.y == self.p2.y {
            // line
            if self.p2.x > self.p1.x {
                return (self.p1.x..self.p2.x + 1)
                    .map(|x_coord| Point {
                        x: x_coord,
                        y: self.p1.y,
                    })
                    .collect();
            } else {
                return (self.p2.x..self.p1.x + 1)
                    .map(|x_coord| Point {
                        x: x_coord,
                        y: self.p1.y,
                    })
                    .collect();
            }
        } else {
            vec![]
        }
    }

    fn diagonal(&self) -> Vec<Point> {
        let slope: i32 =
            ((self.p2.y as i32) - (self.p1.y as i32)) / ((self.p2.x as i32) - (self.p1.x as i32)); // should be 1 or -1
        let q: i32 = ((-slope * self.p1.x as i32) + self.p1.y as i32) as i32;
        if self.p1.x < self.p2.x {
            return (self.p1.x..self.p2.x + 1)
                .map(|x| Point {
                    x,
                    y: (slope * x as i32 + q as i32) as u32,
                })
                .collect();
        } else {
            return (self.p2.x..self.p1.x + 1)
                .rev()
                .map(|x| Point {
                    x,
                    y: ((slope * x as i32) + q as i32) as u32,
                })
                .collect();
        }
    }

    fn line_or_column_or_diagonal(&self) -> Vec<Point> {
        let line_or_column = self.line_or_column();
        if !line_or_column.is_empty() {
            return line_or_column;
        } else {
            let delta_x = (self.p1.x as i32 - self.p2.x as i32).abs();
            let delta_y = (self.p1.y as i32 - self.p2.y as i32).abs();
            if delta_x == delta_y {
                // compute diagonal
                self.diagonal()
            } else {
                vec![]
            }
        }
    }
}

fn part_one(lines: &Vec<Line>) -> u32 {
    let mut hash: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        for point in line.line_or_column() {
            if hash.contains_key(&point) {
                let v = hash.get_mut(&point).unwrap();
                *v += 1;
            } else {
                hash.insert(point, 1);
            }
        }
    }
    hash.values().filter(|x| **x >= 2).count() as u32
}

fn part_two(lines: &Vec<Line>) -> u32 {
    let mut hash: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        for point in line.line_or_column_or_diagonal() {
            if hash.contains_key(&point) {
                let v = hash.get_mut(&point).unwrap();
                *v += 1;
            } else {
                hash.insert(point, 1);
            }
        }
    }
    hash.values().filter(|x| **x >= 2).count() as u32
}

fn parse_line(line: &str) -> Line {
    let mut split = line.split("->").into_iter();
    let left = split.next().unwrap();
    let left_parsed = left.replace(" ", "");
    let mut left_parsed_split = left_parsed.split(",").into_iter();
    let right = split.next().unwrap();
    let right_parsed = right.replace(" ", "");
    let mut right_parsed_split = right_parsed.split(",").into_iter();
    let x1: u32 = left_parsed_split.next().unwrap().parse().unwrap();
    let y1: u32 = left_parsed_split.next().unwrap().parse().unwrap();
    let x2: u32 = right_parsed_split.next().unwrap().parse().unwrap();
    let y2: u32 = right_parsed_split.next().unwrap().parse().unwrap();
    Line {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    }
}

fn parse_input(filename: &str) -> Vec<Line> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Cannot read input file");
    content.lines().map(|line| parse_line(line)).collect()
}

#[test]
fn part_one_works() {
    let lines = parse_input("src/bin/day5_example.txt");
    assert_eq!(part_one(&lines), 5);
}
#[test]
fn part_two_works() {
    let lines = parse_input("src/bin/day5_example.txt");
    assert_eq!(part_two(&lines), 12);
}
