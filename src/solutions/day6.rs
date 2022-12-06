use std::collections::HashSet;

pub fn day6(input: &str) -> (String, String) {
    let chars = input
        .trim()
        .chars()
        .collect::<Vec<char>>();

    let part1 = distinct_window_end(&chars, 4).to_string();
    let part2 = distinct_window_end(&chars, 14).to_string();

    (part1, part2)
}

fn distinct_window_end(chars: &[char], window_size: usize) -> usize {
    let marker = chars
        .windows(window_size)
        .position(distinct_chars)
        .unwrap();

    marker + window_size
}

fn distinct_chars(chars: &[char]) -> bool {
    let len = chars.len();
    let set: HashSet<&char> = HashSet::from_iter(chars.iter());

    set.len() == len
}

#[cfg(test)]
mod tests {
    static INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test_day6() {
        let (part1, part2) = super::day6(INPUTS[0]);

        assert_eq!(part1, "7");
        assert_eq!(part2, "19");

        let (part1, part2) = super::day6(INPUTS[1]);

        assert_eq!(part1, "5");
        assert_eq!(part2, "23");

        let (part1, part2) = super::day6(INPUTS[2]);

        assert_eq!(part1, "6");
        assert_eq!(part2, "23");

        let (part1, part2) = super::day6(INPUTS[3]);

        assert_eq!(part1, "10");
        assert_eq!(part2, "29");

        let (part1, part2) = super::day6(INPUTS[4]);

        assert_eq!(part1, "11");
        assert_eq!(part2, "26");
    }
}
