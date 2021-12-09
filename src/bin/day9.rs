fn main() {
    let map = parse_input("src/bin/day9_input.txt");
    println!("Result for part one is: {}", part_one(&map));
    // println!("Result for part two is: {}", part_two(&lines));
}

fn is_lowest(map: &Vec<Vec<u32>>, y: usize, x: usize) -> bool {
    let cur = map[y][x];
    let mut adjacent: Vec<u32> = vec![];
    if y as i32 - 1 >= 0 {
        adjacent.push(map[y - 1][x]);
    }
    if y as u32 + 1 < map.len() as u32 {
        adjacent.push(map[y + 1][x]);
    }
    if x as i32 - 1 >= 0 {
        adjacent.push(map[y][x - 1]);
    }
    if x as u32 + 1 < map[0].len() as u32 {
        adjacent.push(map[y][x + 1]);
    }
    if adjacent.iter().all(|x| x > &cur) {
        true
    } else {
        false
    }
}

fn part_one(map: &Vec<Vec<u32>>) -> u32 {
    let mut risk = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_lowest(map, y, x) {
                risk += 1 + map[y][x];
            }
        }
    }
    risk
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("unable to read file");
    let mut map: Vec<Vec<u32>> = vec![];
    content.lines().for_each(|line| {
        // let v: Vec<u32> = vec![];
        let v: Vec<u32> = line
            .chars()
            .map(|c| c.to_string().parse::<u32>().unwrap())
            .collect();
        map.push(v);
    });
    map
}

#[test]
fn part_one_works() {
    let map = parse_input("src/bin/day9_example.txt");
    assert_eq!(part_one(&map), 15);
}
