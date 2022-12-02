pub fn day0(_input: &str) -> (String, String) {
    ("test".to_string(), "solution".to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "input";

    #[test]
    fn test_day0() {
        let (part1, part2) = super::day0(INPUT);

        assert_eq!(part1, "test");
        assert_eq!(part2, "solution");
    }
}
