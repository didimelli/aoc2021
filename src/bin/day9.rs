fn main() {
    let mut map = parse_input("src/bin/day9_input.txt");
    println!("Result for part one is: {}", part_one(&mut map));
    println!("Result for part two is: {}", part_two(&mut map));
}

fn is_lowest(map: &mut Vec<Vec<Location>>, y: usize, x: usize) -> bool {
    let cur = map[y][x];
    let adjacent = find_adjacent(map, y, x, false);
    if adjacent.iter().all(|loc| loc.h > cur.h) {
        true
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Location {
    h: u32,
    y: usize,
    x: usize,
    checked: bool,
}

fn find_adjacent(
    map: &mut Vec<Vec<Location>>,
    y: usize,
    x: usize,
    check_visited: bool,
) -> Vec<Location> {
    let mut adjacent: Vec<Location> = vec![];
    if y as i32 - 1 >= 0 {
        if check_visited {
            if !map[y - 1][x].checked {
                adjacent.push(map[y - 1][x]);
            }
            map[y - 1][x].checked = true;
        } else {
            adjacent.push(map[y - 1][x]);
        }
    }
    if y as u32 + 1 < map.len() as u32 {
        if check_visited {
            if !map[y + 1][x].checked {
                adjacent.push(map[y + 1][x]);
            }
            map[y + 1][x].checked = true;
        } else {
            adjacent.push(map[y + 1][x]);
        }
    }
    if x as i32 - 1 >= 0 {
        if check_visited {
            if !map[y][x - 1].checked {
                adjacent.push(map[y][x - 1]);
            }
            map[y][x - 1].checked = true
        } else {
            adjacent.push(map[y][x - 1]);
        };
    }
    if x as u32 + 1 < map[0].len() as u32 {
        if check_visited {
            if !map[y][x + 1].checked {
                adjacent.push(map[y][x + 1]);
            }
            map[y][x + 1].checked = true
        } else {
            adjacent.push(map[y][x + 1]);
        };
    }
    adjacent
}

fn find_basin_size(
    map: &mut Vec<Vec<Location>>,
    // current: &Location,
    cur_y: usize,
    cur_x: usize,
    prev_location: Option<&Location>,
) -> u32 {
    // let lowest = map[y][x];
    // let mut size = 0;
    let mut adjacent = find_adjacent(map, cur_y, cur_x, true);
    let current = map[cur_y][cur_x];
    // size += 1;
    let mut cur_size = 1;
    if let Some(prev) = prev_location {
        // this should remove previous location
        // adjacent.drain_filter(|loc| loc.x == prev.x && loc.y == prev.y);
        if adjacent.contains(prev) {
            adjacent.remove(
                adjacent
                    .iter()
                    .position(|loc| loc.x == prev.x && loc.y == prev.y)
                    .expect("Prev location not found!"),
            );
        }
    }
    if adjacent.iter().all(|loc| loc.h == 9) {
        return cur_size;
    }
    for adj in adjacent {
        // do not go to adjacent point if its height is 9!
        if adj.h != 9 {
            cur_size += find_basin_size(map, adj.y, adj.x, Some(&current));
        }
    }
    cur_size
}

fn part_two(map: &mut Vec<Vec<Location>>) -> u32 {
    let mut sizes: Vec<u32> = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_lowest(map, y, x) {
                sizes.push(find_basin_size(map, y, x, None));
            }
        }
    }
    sizes.sort();
    sizes[sizes.len() - 3] * sizes[sizes.len() - 2] * sizes[sizes.len() - 1]
}

fn part_one(map: &mut Vec<Vec<Location>>) -> u32 {
    let mut risk = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_lowest(map, y, x) {
                risk += 1 + map[y][x].h;
            }
        }
    }
    risk
}

fn parse_input(filename: &str) -> Vec<Vec<Location>> {
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
    let loc_map: Vec<Vec<Location>> = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, h)| Location {
                    h: *h,
                    y,
                    x,
                    checked: false,
                })
                .collect()
        })
        .collect();
    loc_map
}

#[test]
fn part_one_works() {
    let mut map = parse_input("src/bin/day9_example.txt");
    assert_eq!(part_one(&mut map), 15);
}

#[test]
fn part_two_works() {
    let mut map = parse_input("src/bin/day9_example.txt");
    assert_eq!(part_two(&mut map), 1134);
}
