use std::ops::RangeInclusive;

pub fn day4(input: &str) -> (String, String) {
    let pairs: Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let vec: Vec<&str> = line.split(',').collect();
            (vec[0], vec[1])
        })
        .map(|(left, right)| (parse_range(left), parse_range(right)))
        .collect();

    let part1 = pairs
        .iter()
        .filter(|(left, right)| fully_contains(left, right))
        .count()
        .to_string();

    let part2 = pairs
        .iter()
        .filter(|(left, right)| overlaps(left, right))
        .count()
        .to_string();

    (part1, part2)
}

fn fully_contains(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    // a             a
    // |-------------|
    //    |------|
    //    b      b
    let ov1 = b.start() >= a.start() && b.end() <= a.end();

    // b             b
    // |-------------|
    //    |------|
    //    a      a
    let ov2 = a.start() >= b.start() && a.end() <= b.end();

    ov1 || ov2
}

fn overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    //  a      a
    //  |------|
    //      |------|
    //      b      b
    let ov1 = b.start() <= a.end() && a.start() <= b.start();

    //  b      b
    //  |------|
    //      |------|
    //      a      a
    let ov2 = a.start() <= b.end() && b.start() <= a.start();

    ov1 || ov2
}

fn parse_range(range: &str) -> RangeInclusive<i32> {
    let parsed = range
        .split('-')
        .map(|int| int.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    parsed[0]..=parsed[1]
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";

    #[test]
    fn test_day4() {
        let (part1, part2) = super::day4(INPUT);

        assert_eq!(part1, "2");
        assert_eq!(part2, "4");
    }
}
