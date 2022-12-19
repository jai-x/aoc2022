use std::collections::HashSet;

pub fn day3(input: &str) -> (String, String) {
    let rucksacks = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let part1 = part1(rucksacks.clone());
    let part2 = part2(rucksacks);

    (part1, part2)
}

fn part1(rucksacks: Vec<Vec<char>>) -> String {
    rucksacks
        .into_iter()
        .map(|mut chars| {
            let len = chars.len();

            (chars.split_off(len / 2), chars)
        })
        .map(|(right, left)| {
            let left: HashSet<char> = HashSet::from_iter(left.into_iter());
            let right: HashSet<char> = HashSet::from_iter(right.into_iter());

            *left
                .intersection(&right)
                .cloned()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
        })
        .map(priority)
        .sum::<u32>()
        .to_string()
}

fn part2(rucksacks: Vec<Vec<char>>) -> String {
    rucksacks
        .chunks_exact(3)
        .map(|chunk| {
            let one: HashSet<&char> = HashSet::from_iter(chunk[0].iter());
            let two: HashSet<&char> = HashSet::from_iter(chunk[1].iter());
            let thr: HashSet<&char> = HashSet::from_iter(chunk[2].iter());

            let one_two = one.intersection(&two).cloned().collect::<HashSet<&char>>();
            *thr.intersection(&one_two).cloned().collect::<Vec<&char>>()[0]
        })
        .map(priority)
        .sum::<u32>()
        .to_string()
}

fn priority(ch: char) -> u32 {
    match ch {
        'A'..='Z' => ch as u32 - 64 + 26,
        'a'..='z' => ch as u32 - 96,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        ijqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn test_day3() {
        let (part1, part2) = super::day3(INPUT);

        assert_eq!(part1, "157");
        assert_eq!(part2, "70");
    }
}
