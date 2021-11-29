pub mod day1 {
    pub fn return1() -> u32 {
        return 1;
    }
}
#[cfg(test)]
mod tests {
    use crate::day1::day1::return1;
    #[test]
    fn actually_returns1() {
        assert_eq!(return1(), 1);
    }
}
