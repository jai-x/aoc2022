use std::collections::HashSet;

pub fn day10(input: &str) -> (String, String) {
    let instructions: Vec<(&str, Option<isize>)> = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| match parts.as_slice() {
            [instr] => (*instr, None),
            [instr, num] => (*instr, num.parse::<isize>().ok()),
            _ => panic!(),
        })
        .collect();

    let interesting_cycles: HashSet<isize> = HashSet::from([20, 60, 100, 140, 180, 220]);
    let line_cycles: HashSet<isize> = HashSet::from([40, 80, 120, 160, 200, 240]);

    let mut cycles: Vec<(isize, isize)> = Vec::new();
    cycles.push((1, 1));

    for instr in instructions {
        let &(c, x) = cycles.last().unwrap();

        match instr {
            ("noop", _) => {
                cycles.push((c + 1, x));
            }
            ("addx", Some(v)) => {
                cycles.push((c + 1, x));
                cycles.push((c + 2, x + v));
            }
            _ => panic!(),
        }
    }

    let part1 = cycles
        .iter()
        .filter(|(c, _)| interesting_cycles.contains(c))
        .map(|&(c, x)| c * x)
        .sum::<isize>()
        .to_string();

    let mut part2: String = String::new();
    part2.push('\n');

    for (c, x) in cycles {
        let p = c % 40;
        let out = if p == x || p == x + 1 || p == x + 2 {
            '█'
        } else {
            ' '
        };

        part2.push(out);
        if line_cycles.contains(&c) {
            part2.push('\n');
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop";

    #[test]
    fn test_day10() {
        let (part1, _) = super::day10(INPUT);

        assert_eq!(part1, "13140");
    }
}
