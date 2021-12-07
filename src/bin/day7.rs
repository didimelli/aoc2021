fn main() {
    let crabs = parse_input("src/bin/day7_input.txt");
    println!("Result for part one is: {}", part_one(&crabs));
    println!("Result for part two is: {}", part_two(&crabs));
}

fn part_one(crabs: &Vec<u32>) -> u32 {
    let mut fuel: Vec<u32> = vec![];
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    for goal in *min..*max {
        let mut consumed = 0;
        for crab in crabs {
            consumed += (*crab as i32 - goal as i32).abs() as u32;
        }
        fuel.push(consumed);
    }
    *fuel.iter().min().unwrap()
}

fn part_two(crabs: &Vec<u32>) -> u32 {
    let mut fuel: Vec<u32> = vec![];
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    for goal in *min..*max {
        let mut consumed = 0;
        for crab in crabs {
            let dist = (*crab as i32 - goal as i32).abs() as u32;
            consumed += (0..dist + 1).fold(0, |acc, x| acc + x);
        }
        fuel.push(consumed);
    }
    *fuel.iter().min().unwrap()
}

fn parse_input(filename: &str) -> Vec<u32> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Unable to open file");
    let line = content.lines().next().unwrap();
    line.split(",").map(|x| x.parse().unwrap()).collect()
}

#[test]
fn part_one_works() {
    let crabs = parse_input("src/bin/day7_example.txt");
    assert_eq!(part_one(&crabs), 37);
}
#[test]
fn part_two_works() {
    let crabs = parse_input("src/bin/day7_example.txt");
    assert_eq!(part_two(&crabs), 168);
}
