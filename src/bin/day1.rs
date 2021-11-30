fn main() {
    println!("DAY 1!")
}

fn return1() -> u32 {
    1
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
    assert_eq!(return1(), 1);
}
