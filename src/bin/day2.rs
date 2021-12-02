fn main() {
    let input = parse_input("src/bin/day2_input.txt");
    println!("Result for part one is: {}", part_one(&input));
    println!("Result for part two is: {}", part_two(&input));
}

fn parse_input(filename: &str) -> Vec<String> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Something wrong reading the file");
    parse_lines(&content)
}

fn parse_lines(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_owned()).collect()
}

fn parse_line(input: &str) -> (&str, u32) {
    let split: Vec<&str> = input.split(' ').collect();
    let command: &str = split[0];
    let strength: u32 = split[1].parse().unwrap();
    (command, strength)
}

fn part_one(input: &Vec<String>) -> u32 {
    let parsed_lines: Vec<(&str, u32)> = input.iter().map(|x| parse_line(x)).collect();
    let mut horizontal = 0;
    let mut vertical = 0;
    parsed_lines.iter().for_each(|x| match x.0 {
        "forward" => horizontal += x.1,
        "up" => vertical -= x.1,
        "down" => vertical += x.1,
        _ => panic!("Unknown command {}", x.0),
    });
    horizontal * vertical
}

fn part_two(input: &Vec<String>) -> u32 {
    let parsed_lines: Vec<(&str, u32)> = input.iter().map(|x| parse_line(x)).collect();
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    parsed_lines.iter().for_each(|x| match x.0 {
        "forward" => {
            horizontal += x.1;
            vertical += aim * x.1;
        }
        "up" => aim -= x.1,
        "down" => aim += x.1,
        _ => panic!("Unknown command {}", x.0),
    });
    horizontal * vertical
}

#[test]
fn part_one_works() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let input_vec = parse_lines(input);
    assert_eq!(part_one(&input_vec), 150);
}
#[test]
fn part_two_works() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let input_vec = parse_lines(input);
    assert_eq!(part_two(&input_vec), 900);
}
