mod day1;

#[cfg(test)]
mod tests {
    use crate::day1::day1::return1;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_eq!(return1(), 1);
    }
}
