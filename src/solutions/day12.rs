use std::collections::{VecDeque, HashSet, HashMap};

const OFFSETS: [(isize,isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug)]
struct HeightMap {
    heights: HashMap<(isize, isize), char>,
    start: (isize, isize),
    goal: (isize, isize),
}

impl HeightMap {
    fn parse(input: &str) -> HeightMap {
        let mut heights: HashMap<(isize, isize), char> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| ((x as isize, y as isize), ch))
            })
            .collect();


        let mut start: (isize, isize) = (0, 0);
        let mut goal: (isize, isize) = (0, 0);

        for (k, v) in heights.iter() {
            match v {
                'S' => start = *k,
                'E' => goal = *k,
                _ => (),
            }
        }

        heights.entry(start).and_modify(|ch| *ch = 'a');
        heights.entry(goal).and_modify(|ch| *ch = 'z');

        HeightMap { heights, start, goal }
    }

    // Bad Djikstra's :)
    fn find_distance_to_end(&self) -> Option<isize> {
        let mut open: VecDeque<((isize, isize), isize)> = VecDeque::new();

        open.push_back((self.start, 0));

        let mut closed: HashSet<(isize, isize)> = HashSet::new();

        while let Some((current_coords, current_dist)) = open.pop_front() {
            // Don't reconsider coords seen
            if closed.contains(&current_coords) {
                continue;
            }

            if current_coords == self.goal {
                return Some(current_dist);
            }

            closed.insert(current_coords);

            let neighbour_coords: Vec<(isize, isize)> = OFFSETS
                .map(|offset| (offset.0 + current_coords.0, offset.1 + current_coords.1))
                .into_iter()
                // Remove out of bounds coords
                .filter(|neighbour| self.heights.contains_key(neighbour))
                .collect();

            let current_height = *self.heights.get(&current_coords).unwrap() as u8;

            for n in neighbour_coords {
                let n_height = *self.heights.get(&n).unwrap() as u8;

                if n_height <= current_height + 1 {
                    // Abusing properties of square grid and constant distance of 1
                    open.push_back((n, current_dist + 1));
                }
            }
        }

        None
    }

    // Bad Djikstra's :)
    fn find_distance_to_first_a(&self) -> Option<isize> {
        let mut open: VecDeque<((isize, isize), isize)> = VecDeque::new();

        open.push_back((self.goal, 0));

        let mut closed: HashSet<(isize, isize)> = HashSet::new();

        while let Some((current_coords, current_dist)) = open.pop_front() {
            // Don't reconsider coords seen
            if closed.contains(&current_coords) {
                continue;
            }

            if self.heights.get(&current_coords).unwrap() == &'a' {
                return Some(current_dist);
            }

            closed.insert(current_coords);

            let neighbour_coords: Vec<(isize, isize)> = OFFSETS
                .map(|offset| (offset.0 + current_coords.0, offset.1 + current_coords.1))
                .into_iter()
                // Remove out of bounds coords
                .filter(|neighbour| self.heights.contains_key(neighbour))
                .collect();

            let current_height = *self.heights.get(&current_coords).unwrap() as u8;

            for n in neighbour_coords {
                let n_height = *self.heights.get(&n).unwrap() as u8;

                if current_height <= n_height + 1 {
                    // Abusing properties of square grid and constant distance of 1
                    open.push_back((n, current_dist + 1));
                }
            }
        }

        None
    }
}


pub fn day12(input: &str) -> (String, String) {
    let height_map = HeightMap::parse(input);

    let part1 = height_map.find_distance_to_end().unwrap().to_string();
    let part2 = height_map.find_distance_to_first_a().unwrap().to_string();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    const INPUT_LINES: [&str; 5] = [
        "Sabqponm",
        "abcryxxl",
        "accszExk",
        "acctuvwj",
        "abdefghi",
    ];

    #[test]
    fn test_day0() {
        let input = INPUT_LINES.join("\n");
        let (part1, part2) = super::day12(&input);

        assert_eq!(part1, "31");
        assert_eq!(part2, "29");
    }
}
