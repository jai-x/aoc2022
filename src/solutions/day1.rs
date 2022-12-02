pub fn day1(input: &str) -> (String, String) {
    let mut calories: Vec<i32> = Vec::new();

    for elf_group in input.split("\n\n") {
        let elf_group_calories: i32 = elf_group
            .lines()
            .map(|val| val.trim().parse::<i32>().unwrap())
            .sum();

        calories.push(elf_group_calories);
    }

    calories.sort(); // sorts in ascending order
    calories.reverse();

    let part1 = calories.first().unwrap().to_string();
    let part2 = calories.iter().take(3).sum::<i32>().to_string();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    static INPUT: &str = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    ";

    #[test]
    fn test_day1() {
        let (part1, part2) = super::day1(INPUT.trim());

        assert_eq!(part1, "24000");
        assert_eq!(part2, "45000");
    }
}
