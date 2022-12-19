pub fn day6(input: &str) -> (String, String) {
    let chars = input.trim().chars().collect::<Vec<char>>();

    let part1 = distinct_window_end(&chars, 4).to_string();
    let part2 = distinct_window_end(&chars, 14).to_string();

    (part1, part2)
}

fn distinct_window_end(chars: &[char], window_size: usize) -> usize {
    let marker = chars.windows(window_size).position(distinct_chars).unwrap();

    marker + window_size
}

fn distinct_chars(chars: &[char]) -> bool {
    // Since the input set of characters is lowercase ascii,
    // each character can be converted to a bitmask represention
    // that fits inside a u32 as there's only 26 letters :)
    let mut bits: u32 = 0;
    for ch in chars {
        let char_mask: u32 = 1 << ((*ch as u32) - 97);
        // bitwise or
        bits |= char_mask;
    }

    // If the set of chars is unique, then each char mask should
    // have only set one bit, and the popcount should be equal to
    // the number of chars input
    bits.count_ones() == chars.len() as u32
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
