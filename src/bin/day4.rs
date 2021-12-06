fn main() {
    let (drawn, mut boards) = parse_input("src/bin/day4_input.txt");
    println!("Result for part one is: {}", part_one(&drawn, &mut boards));
    // println!("Result for part two is: {}", part_two(&input));
}

#[derive(Debug, Clone, Copy)]
struct Board {
    // numbers: Vec<Number>,
    values: [u32; 25],
    drawn: [u8; 25],
    winning: Option<[u32; 5]>,
}

impl Board {
    fn new_drawn(&mut self, number: u32) {
        for (i, x) in self.values.iter().enumerate() {
            if *x == number {
                // x.drawn = true;
                self.drawn[i] = 1;
            }
        }
    }

    fn is_board_winning(&mut self) -> bool {
        let mut is_winning = false;
        for idx in 0..5 {
            // loop column
            let column = self.get_column_by_index(idx);
            if self.is_winning_column_by_index(idx) {
                is_winning = true;
                self.winning = Some(column);
                break;
            }
        }
        for idx in (0..25).step_by(5) {
            // line loop
            let line = self.get_line_by_index(idx);
            if self.is_winning_line_by_index(idx) {
                is_winning = true;
                self.winning = Some(line);
                break;
            }
        }
        is_winning
    }

    fn is_winning_line_by_index(&self, idx: u32) -> bool {
        [
            self.drawn[idx as usize],
            self.drawn[(idx + 1) as usize],
            self.drawn[(idx + 2) as usize],
            self.drawn[(idx + 3) as usize],
            self.drawn[(idx + 4) as usize],
        ]
        .iter()
        .all(|x| *x == 1)
    }

    fn is_winning_column_by_index(&self, idx: u32) -> bool {
        [
            self.drawn[idx as usize],
            self.drawn[(idx + 5) as usize],
            self.drawn[(idx + 10) as usize],
            self.drawn[(idx + 15) as usize],
            self.drawn[(idx + 20) as usize],
        ]
        .iter()
        .all(|x| *x == 1)
    }
    fn get_line_by_index(&self, idx: u32) -> [u32; 5] {
        // check lines: indeces goes from continuously 5 by 5 (0-1-2-3-4, 5-6-7-8-9, ...)
        [
            self.values[idx as usize],
            self.values[(idx + 1) as usize],
            self.values[(idx + 2) as usize],
            self.values[(idx + 3) as usize],
            self.values[(idx + 4) as usize],
        ]
    }
    fn get_column_by_index(&self, idx: u32) -> [u32; 5] {
        // check columns: indeces goes from jumping 5 by 5 (0-5-10-15-20, 1-6-11-16-21, ...)
        [
            self.values[idx as usize],
            self.values[(idx + 5) as usize],
            self.values[(idx + 10) as usize],
            self.values[(idx + 15) as usize],
            self.values[(idx + 20) as usize],
        ]
    }

    fn get_unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for (idx, value) in self.drawn.iter().enumerate() {
            if *value == 0 {
                sum += self.values[idx];
            }
        }
        sum
    }

    fn from_chunk(chunk: &str) -> Self {
        let mut values: [u32; 25] = [0; 25];
        let drawn: [u8; 25] = [0; 25];
        let mut idx = 0;
        // println!("{}", chunk);
        chunk.lines().for_each(|line| {
            if !line.is_empty() {
                // println!("{}", line);
                line.split(" ").for_each(|x| {
                    if !x.is_empty() {
                        let parsed = x.trim_start().parse();
                        if let Ok(p) = parsed {
                            values[idx] = p;
                            idx += 1
                        } else {
                            panic!("{} is not a number!", x);
                        }
                    }
                })
            }
        });
        // check that numbers.len() == 25
        assert_eq!(idx, 25);
        Self {
            values,
            drawn,
            winning: None,
        }
    }
}

fn part_one(drawn: &Vec<u32>, boards: &mut Vec<Board>) -> u32 {
    let mut winning_sum: Option<u32> = None;
    let mut winning_number: Option<u32> = None;
    'outer: for d in drawn {
        for board in &mut *boards {
            // if number is in board, tick it as drawn!
            board.new_drawn(*d);
            // check if board is winning
            if board.is_board_winning() {
                winning_sum = Some(board.get_unmarked_sum());
                println!("{:?} - {}", board, d);
                winning_number = Some(*d);
                break 'outer;
            }
        }
    }
    let sum: u32 = winning_sum.unwrap();
    sum * winning_number.unwrap()
}

fn parse_input(filename: &str) -> (Vec<u32>, Vec<Board>) {
    //(Vec<u32>, Vec<Board>) {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Cannot read input file");
    let first_line = content.lines().next();
    let drawn: Vec<u32> = first_line
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let boards_raw: Vec<&str> = content.split("\n\n").collect();
    let boards: Vec<Board> = boards_raw[1..]
        .iter()
        .map(|b| Board::from_chunk(*b))
        .collect();
    (drawn, boards)
}

#[test]
fn part_one_works() {
    let (drawn, mut boards) = parse_input("src/bin/day4_example.txt");
    assert_eq!(part_one(&drawn, &mut boards), 4512);
}

// #[test]
// fn part_two_works() {
//     assert_eq!(
//         part_two(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
//         5
//     );
// }
