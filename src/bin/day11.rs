fn main() {
    let map = parse_input("src/bin/day11_input.txt");
    println!(
        "Result for part one is: {}",
        part_one(&mut map.clone(), 100)
    );
    println!("Result for part two is: {}", part_two(&mut map.clone()));
}

#[derive(Clone, Copy, Debug)]
struct Octopus {
    energy: u8,
    y: usize,
    x: usize,
    flashed: bool,
}

impl Octopus {
    fn increase_energy(&mut self) {
        self.energy += 1;
    }

    fn should_flash(&self) -> bool {
        self.energy > 9 && !self.flashed // if already flashed, should not do it again!
    }

    fn reset(&mut self) {
        if self.flashed {
            self.energy = 0;
            self.flashed = false;
        }
    }

    fn adjacents(&self, max_y: usize, max_x: usize) -> Vec<(usize, usize)> {
        let mut adj: Vec<(i32, i32)> = vec![];
        let y = self.y as i32;
        let x = self.x as i32;
        /* let up_left =  */
        adj.push((y - 1, x - 1));
        /* let up =  */
        adj.push((y - 1, x));
        /* let up_right =  */
        adj.push((y - 1, x + 1));
        /* let right =  */
        adj.push((y, x + 1));
        /* let down_right =  */
        adj.push((y + 1, x + 1));
        /* let down =  */
        adj.push((y + 1, x));
        /* let down_left =  */
        adj.push((y + 1, x - 1));
        /* let left =  */
        adj.push((y, x - 1));
        adj.iter()
            .filter(|(y, x)| {
                // check that y and x are > 0
                // check that y < max_y and x < max_x (length - 1)
                *y >= 0 && *y < max_y as i32 && *x >= 0 && *x < max_x as i32
            })
            .map(|(y, x)| (*y as usize, *x as usize))
            .collect()
    }
}

fn flash(map: &mut Vec<Vec<Octopus>>, y: usize, x: usize) -> u32 {
    if !map[y][x].should_flash() {
        return 0;
    }
    map[y][x].flashed = true;
    let adjacent = map[y][x].adjacents(map.len(), map[0].len());
    let mut count = 1;
    for (y, x) in &adjacent {
        map[*y][*x].increase_energy();
    }
    for (y, x) in &adjacent {
        count += flash(map, *y, *x);
    }
    count
}

fn step(map: &mut Vec<Vec<Octopus>>) -> u32 {
    let mut flashes = 0;
    // first, increase energy to all
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            map[y][x].increase_energy();
        }
    }
    // if energy == 9 -> flashes!
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // println!("Flashing {}-{}", y, x);
            flashes += flash(map, y, x);
        }
    }
    // reset flashes!
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            map[y][x].reset();
        }
    }
    let mut repr = String::new();
    for y in 0..map.len() {
        let mut line = String::new();
        for x in 0..map[0].len() {
            line += &map[y][x].energy.to_string();
        }
        line += "\n";
        repr += &line;
    }
    println!("{}", repr);
    flashes
}

fn part_one(map: &mut Vec<Vec<Octopus>>, steps: u8) -> u32 {
    let mut flashes = 0;
    for s in 0..steps {
        flashes += step(map);
        println!("Step {} -> Flashes {}", s + 1, flashes);
    }
    flashes
}

fn part_two(map: &mut Vec<Vec<Octopus>>) -> u32 {
    let mut steps = 0;
    for s in 0.. {
        steps = s;
        let first = map[0][0].energy;
        let is_synchronized = map
            .iter()
            .all(|line| line.iter().all(|oct| oct.energy == first));
        if is_synchronized {
            break;
        }
        step(map);
        println!("Step {}", s + 1);
        // check if synchronized
    }
    steps
}

fn parse_input(filename: &str) -> Vec<Vec<Octopus>> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Something wrong reading the file");
    content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, o)| Octopus {
                    energy: o.to_string().parse().unwrap(),
                    y,
                    x,
                    flashed: false,
                })
                .collect()
        })
        .collect()
}

#[test]
fn part_one_works() {
    let mut map = parse_input("src/bin/day11_example.txt");
    assert_eq!(part_one(&mut map, 100), 1656);
}

#[test]
fn part_two_works() {
    let mut map = parse_input("src/bin/day11_example.txt");
    assert_eq!(part_two(&mut map), 195);
}
