pub fn day2(input: &str) -> (String, String) {
    let out = input
        .trim()
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            (score(trimmed), outcome(trimmed))
        })
        .reduce(|acc, el| (acc.0 + el.0, acc.1 + el.1))
        .unwrap();

    (out.0.to_string(), out.1.to_string())
}

fn score(pair: &str) -> i32 {
    match pair {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,

        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,

        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,

        _ => 0,
    }
}

fn outcome(pair: &str) -> i32 {
    match pair {
        "A X" => 0 + 3,
        "B X" => 0 + 1,
        "C X" => 0 + 2,

        "A Y" => 3 + 1,
        "B Y" => 3 + 2,
        "C Y" => 3 + 3,

        "A Z" => 6 + 2,
        "B Z" => 6 + 3,
        "C Z" => 6 + 1,

        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
        A Y
        B X
        C Z
    ";

    #[test]
    fn test_day2() {
        let (part1, part2) = super::day2(INPUT);

        assert_eq!(part1, "15");
        assert_eq!(part2, "12");
    }
}
