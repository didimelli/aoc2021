fn main() {
    let input = parse_input("src/bin/day3_input.txt");
    println!("Result for part one is: {}", part_one(&input));
    println!("Result for part two is: {}", part_two(&input));
}

fn parse_input(filename: &str) -> Vec<Vec<u8>> {
    use std::fs;
    let content = fs::read_to_string(filename).expect("Something wrong reading the file");
    parse_lines(&content)
}

fn parse_lines(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Unexpected char {}", c),
                })
                .collect()
        })
        .collect()
}

fn part_one(input: &Vec<Vec<u8>>) -> u32 {
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for i in 0..input[0].len() {
        let mut count0 = 0;
        let mut count1 = 0;
        input.iter().for_each(|x| match x[i] {
            0 => count0 += 1,
            1 => count1 += 1,
            _ => panic!("Should not be here!"),
        });
        if count1 > count0 {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
    }
    let gamma_rate_as_decimal = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate_as_decimal = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();
    (gamma_rate_as_decimal * epsilon_rate_as_decimal) as u32
}

fn part_two(input: &Vec<Vec<u8>>) -> u32 {
    let mut item_iter = 0..input[0].len();
    let mut input_ox = input.clone();
    let mut input_co2 = input.clone();
    while let Some(idx) = item_iter.next() {
        // need to check length of inputs also here!
        if input_ox.len() != 1 {
            let vertical_ox = get_vertical_slice(&input_ox, idx);
            match get_most_common_item(&vertical_ox) {
                1 => {
                    // update internal input ox
                    input_ox = input_ox
                        .iter()
                        .filter(|&x| x[idx as usize] == 1)
                        .cloned()
                        .collect();
                }
                0 => {
                    // update internal input ox
                    input_ox = input_ox
                        .iter()
                        .filter(|&x| x[idx as usize] == 0)
                        .cloned()
                        .collect();
                }
                2 => {
                    // update internal input ox
                    input_ox = input_ox
                        .iter()
                        .filter(|&x| x[idx as usize] == 1)
                        .cloned()
                        .collect();
                }
                _ => panic!("WTF OX"),
            }
        }
        if input_co2.len() != 1 {
            let vertical_co2 = get_vertical_slice(&input_co2, idx);
            match get_most_common_item(&vertical_co2) {
                1 => {
                    // update internal input co2
                    input_co2 = input_co2
                        .iter()
                        .filter(|&x| x[idx as usize] == 0)
                        .cloned()
                        .collect();
                }
                0 => {
                    // update internal input co2
                    input_co2 = input_co2
                        .iter()
                        .filter(|&x| x[idx as usize] == 1)
                        .cloned()
                        .collect();
                }
                2 => {
                    // update internal input co2
                    input_co2 = input_co2
                        .iter()
                        .filter(|&x| x[idx as usize] == 0)
                        .cloned()
                        .collect();
                }
                _ => panic!("WTF CO2"),
            }
        }
    }
    if input_ox.len() == 1 && input_co2.len() == 1 {
        let ox_as_str: String = input_ox[0].iter().map(|x| x.to_string()).collect();
        let co2_as_str: String = input_co2[0].iter().map(|x| x.to_string()).collect();
        (isize::from_str_radix(ox_as_str.as_str(), 2).unwrap()
            * isize::from_str_radix(co2_as_str.as_str(), 2).unwrap()) as u32
    } else {
        panic!(
            "Lengths are not 1 but {} and {}",
            input_ox.len(),
            input_co2.len()
        );
    }
}

fn get_vertical_slice(input: &Vec<Vec<u8>>, idx: usize) -> Vec<u8> {
    input.iter().map(|x| x[idx]).collect()
}

fn get_most_common_item(input: &Vec<u8>) -> u8 {
    let count = input.iter().filter(|x| **x == 0 as u8).count();
    if count > input.len() / 2 {
        0
    } else if count < input.len() / 2 {
        1
    } else {
        2
    }
}

#[test]
fn part_one_works() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let input_vec = parse_lines(input);
    assert_eq!(part_one(&input_vec), 198);
}

#[test]
fn part_two_works() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let input_vec = parse_lines(input);
    assert_eq!(part_two(&input_vec), 230);
}
