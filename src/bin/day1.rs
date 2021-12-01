fn main() {
    let input = parse_input("src/bin/day1_input.txt");
    println!("Result for part one is: {}", part_one(&input));
    println!("Result for part two is: {}", part_two(&input));
}

fn parse_input(filename: &str) -> Vec<u32> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Something wrong reading the file");
    content
        .lines()
        .map(|x| {
            x.parse()
                .expect(format!("Cannot parse into u32 this {}", x).as_str())
        })
        .collect()
}

fn part_one(input: &Vec<u32>) -> u32 {
    let mut increase_counter = 0;
    for idx in 0..input.len() - 1 {
        if input[idx + 1] > input[idx] {
            increase_counter += 1;
        }
    }
    increase_counter
}

fn part_two(input: &Vec<u32>) -> u32 {
    let mut increase_counter = 0;
    for idx in 0..input.len() - 3 {
        // i actually have to access 6 items from here
        let first_window = input[idx] + input[idx + 1] + input[idx + 2];
        let second_window = input[idx + 1] + input[idx + 2] + input[idx + 3];
        if second_window > first_window {
            increase_counter += 1;
        }
    }
    increase_counter
}

#[test]
fn part_one_works() {
    assert_eq!(
        part_one(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}
#[test]
fn part_two_works() {
    assert_eq!(
        part_two(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        5
    );
}
