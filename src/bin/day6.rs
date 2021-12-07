use std::collections::HashMap;

fn main() {
    // let lines = parse_input("src/bin/day5_input.txt");
    let fishes = parse_input("src/bin/day6_input.txt");
    println!(
        "Result for part one is: {}",
        part_one(&mut fishes.clone(), 80)
    );
    println!(
        "Result for part two is: {}",
        part_two(&mut fishes.clone(), 256)
    );
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Fish {
    time_until_new_fish: u32,
}

impl Fish {
    fn new() -> Self {
        Self {
            time_until_new_fish: 8,
        }
    }

    fn grow_older(&mut self) {
        if self.time_until_new_fish > 0 {
            self.time_until_new_fish -= 1;
        } else {
            self.time_until_new_fish = 6
        }
    }

    fn older_fish(&self) -> Fish {
        if self.time_until_new_fish > 0 {
            Fish {
                time_until_new_fish: self.time_until_new_fish - 1,
            }
        } else {
            Fish {
                time_until_new_fish: 6,
            }
        }
    }

    fn should_spawn(&self) -> bool {
        if self.time_until_new_fish == 0 {
            true
        } else {
            false
        }
    }
}

fn part_one(fishes: &mut Vec<Fish>, how_many_days: u32) -> u64 {
    for _ in 0..how_many_days {
        // println!("Day {} on {}", day, how_many_days);
        let mut fishes_to_spawn: u32 = 0;
        for fish in &mut *fishes {
            if fish.should_spawn() {
                fishes_to_spawn += 1;
            }
            fish.grow_older();
        }
        // add newly spawned fished to fishes!
        for _ in 0..fishes_to_spawn {
            fishes.push(Fish::new());
        }
    }
    fishes.len() as u64
}

fn part_two(fishes: &mut Vec<Fish>, how_many_days: u32) -> u64 {
    // i can store the fishes in an hashmap Fish: how_many
    let mut fish_map: HashMap<Fish, u64> = HashMap::new();
    for fish in fishes {
        if fish_map.contains_key(fish) {
            let count = fish_map.get_mut(&fish).unwrap();
            *count += 1;
        } else {
            fish_map.insert(*fish, 1);
        }
    }
    for _ in 0..how_many_days {
        let fish_map_copy: HashMap<Fish, u64> = fish_map.clone();
        let mut tmp_fish_map: HashMap<Fish, u64> = HashMap::new();
        // println!("Day {} on {}", day, how_many_days);
        // get key from copy, update original
        for (fish, count) in fish_map_copy.iter() {
            if fish.should_spawn() {
                tmp_fish_map.insert(Fish::new(), *count);
            }
            match fish.time_until_new_fish {
                0 | 7 => {
                    if tmp_fish_map.contains_key(&fish.older_fish()) {
                        let c = tmp_fish_map.get_mut(&fish.older_fish()).unwrap();
                        *c += count;
                    } else {
                        tmp_fish_map.insert(fish.older_fish(), *count);
                    }
                }
                1 | 2 | 3 | 4 | 5 | 6 | 8 => {
                    tmp_fish_map.insert(fish.older_fish(), *count);
                }
                _ => panic!("Should not be here"),
            }
        }
        fish_map = tmp_fish_map;
    }
    fish_map.values().fold(0, |acc, x| acc + x)
}

fn parse_input(filename: &str) -> Vec<Fish> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Cannot read input file");
    content
        .replace("\n", "")
        .split(",")
        .map(|x| Fish {
            time_until_new_fish: x.parse().unwrap(),
        })
        .collect()
}

#[test]
fn part_one_works_18_days() {
    let mut fishes = parse_input("src/bin/day6_example.txt");
    assert_eq!(part_one(&mut fishes, 18), 26);
}
// #[test]
#[test]
fn part_one_works_80_days() {
    let mut fishes = parse_input("src/bin/day6_example.txt");
    assert_eq!(part_one(&mut fishes, 80), 5934);
}
#[test]
fn part_two_works_256_days() {
    let mut fishes = parse_input("src/bin/day6_example.txt");
    assert_eq!(part_two(&mut fishes, 256), 26984457539);
}
