fn main() {
    part_one(vec![1721, 979, 366, 299, 675, 1456]);
}

fn part_one(input: Vec<u32>) -> u32 {
    // find the two entries that sum to 2020 and return their product
    let result = input
        .iter()
        .find_map(|x| match input.iter().find(|xx| *x + **xx == 2020) {
            Some(xx) => Some(x * xx),
            None => None,
        });
    result.unwrap_or(1)
}

#[test]
fn part_one_works() {
    assert_eq!(part_one(vec![1721, 979, 366, 299, 675, 1456]), 514579);
}
