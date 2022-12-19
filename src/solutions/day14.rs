use std::fmt::{Debug, Formatter, Result};
use std::collections::HashSet;

enum CaveEnd {
    Abyss,
    Floor,
}

struct Cave {
    rocks: HashSet<(usize, usize)>,
    sand: HashSet<(usize, usize)>,
    trail: HashSet<(usize, usize)>,
    abyss: usize,
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut out = String::new();
        out.push_str("Cave {\n");
        out.push_str(&format!("    abyss = {}\n", self.abyss));
        out.push_str("    map = {\n");

        let min_x_rocks = *self.rocks.iter().map(|(x, _)| x).min().unwrap();
        let max_x_rocks = *self.rocks.iter().map(|(x, _)| x).max().unwrap();
        let max_y_rocks = *self.rocks.iter().map(|(_, y)| y).max().unwrap();

        let min_x_sand = *self.sand.iter().map(|(x, _)| x).min().unwrap();
        let max_x_sand = *self.sand.iter().map(|(x, _)| x).max().unwrap();
        let max_y_sand = *self.sand.iter().map(|(_, y)| y).max().unwrap();

        let min_x = min_x_rocks.min(min_x_sand);
        let max_x = max_x_rocks.max(max_x_sand);
        let max_y = max_y_rocks.max(max_y_sand);

        for y in 0..=max_y {
            out.push_str("        ");
            for x in min_x..=max_x {
                let coords = (x, y);
                let ch = if self.rocks.contains(&coords) {
                    '#'
                } else if self.sand.contains(&coords) {
                    'o'
                } else if self.trail.contains(&coords) {
                    '~'
                } else {
                    '.'
                };
                out.push(ch);
            }
            out.push('\n');
        }

        out.push_str("    }\n");
        out.push('}');

        writeln!(f, "{}", out)
    }
}

impl Cave {
    fn parse(input: &str) -> Cave {
        let rocks: HashSet<(usize, usize)> = input
            .lines()
            .flat_map(|line| {
                line.split(" -> ")
                    .map(|pair| pair.split(',').collect::<Vec<&str>>())
                    .map(|pair_vec| {
                        let x = pair_vec[0].parse().unwrap();
                        let y = pair_vec[1].parse().unwrap();
                        (x, y)
                    })
                    .collect::<Vec<(usize,usize)>>()
                    .windows(2)
                    .flat_map(|pair| Self::coord_travel(pair[0], pair[1]))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        let abyss = *rocks.iter().map(|(_, y)| y).max().unwrap();

        Cave {
            rocks,
            abyss,
            sand: HashSet::new(),
            trail: HashSet::new(),
        }
    }

    fn coord_travel(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        let (x1, y1) = start;
        let (x2, y2) = end;

        if x1 == x2 && y1 == y2 {
            return vec![start];
        }

        if x1 == x2 {
            let range = if y1 > y2 { y2..=y1 } else { y1..=y2 };
            return range.map(|y| (x1, y)).collect();
        }

        if y1 == y2 {
            let range = if x1 > x2 { x2..=x1 } else { x1..=x2 };
            return range.map(|x| (x, y1)).collect();
        }

        unreachable!()
    }

    fn drop_sand(&mut self, start: (usize, usize), end: CaveEnd) -> Option<(usize, usize)> {
        let (mut x, mut y) = start;

        self.trail.clear();

        loop {
            match end {
                CaveEnd::Abyss => {
                    if y == self.abyss {
                        return None; // not settled
                    }
                },
                CaveEnd::Floor => {
                    if y == self.abyss + 1 {
                        self.sand.insert((x, y));
                        return Some((x, y)); // settled on floor
                    }
                },
            }

            let below = (x, y + 1);
            let rock_below = self.rocks.contains(&below);
            let sand_below = self.sand.contains(&below);

            if !rock_below && !sand_below {
                y += 1;
                self.trail.insert((x, y));
                continue;
            }

            let below_left = (x - 1, y + 1);
            let rock_below_left = self.rocks.contains(&below_left);
            let sand_below_left = self.sand.contains(&below_left);

            if !rock_below_left && !sand_below_left {
                y += 1;
                x -= 1;
                self.trail.insert((x, y));
                continue;
            }

            let below_right = (x + 1, y + 1);
            let rock_below_right = self.rocks.contains(&below_right);
            let sand_below_right = self.sand.contains(&below_right);

            if !rock_below_right && !sand_below_right {
                y += 1;
                x += 1;
                self.trail.insert((x, y));
                continue;
            }

            self.sand.insert((x, y));
            return Some((x, y)) // settled
        }
    }
}

pub fn day14(input: &str) -> (String, String) {
    let mut cave = Cave::parse(input);

    let start: (usize, usize) = (500, 0);

    loop {
        match cave.drop_sand(start, CaveEnd::Abyss) {
            None => break,
            Some(_) => continue,
        }
    }

    let part1 = cave.sand.len().to_string();

    loop {
        match cave.drop_sand(start, CaveEnd::Floor) {
            None => unreachable!(),
            Some(settled_at) => {
                if settled_at == start {
                    break;
                }
            }
        }
    }

    let part2 = cave.sand.len().to_string();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    const INPUT_LINES: [&str; 2] = [
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    #[test]
    fn test_day14() {
        let input = INPUT_LINES.join("\n");
        let (part1, part2) = super::day14(&input);

        assert_eq!(part1, "24");
        assert_eq!(part2, "93");
    }
}
