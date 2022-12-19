struct Map {
    sensor_ranges: Vec::<((isize, isize), usize)>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let sensor_ranges: Vec::<((isize, isize), usize)> = input
            .lines()
            .map(|line| {
                let (s_part, b_part) = line.split_once(": ").unwrap();

                let s_xy = s_part.trim_start_matches("Sensor at ");
                let b_xy = b_part.trim_start_matches("closest beacon is at ");

                let sensor = Self::parse_xy_str(s_xy);
                let beacon = Self::parse_xy_str(b_xy);

                (sensor, beacon)
            })
            .map(|(sensor, beacon)| (sensor, Self::manhatten(sensor, beacon)))
            .collect();

        Map { sensor_ranges }
    }

    fn parse_xy_str(xy_str: &str) -> (isize, isize) {
        let (x_str, y_str) = xy_str.split_once(", ").unwrap();

        let x = x_str.trim_start_matches("x=").parse().unwrap();
        let y = y_str.trim_start_matches("y=").parse().unwrap();

        (x, y)
    }

    fn manhatten(a: (isize, isize), b: (isize, isize)) -> usize {
        let (x_a, y_a) = a;
        let (x_b, y_b) = b;

        x_a.abs_diff(x_b) + y_a.abs_diff(y_b)
    }

    fn intersecting_x_ranges(&self, at_y: isize, clamp: Option<isize>) -> Vec<(isize, isize)> {
        let mut ranges: Vec<(isize, isize)> = self.sensor_ranges
            .iter()
            .filter(|(sensor, sensor_to_beacon)| {
                let closest_y   = (sensor.0, at_y);
                let sensor_to_y = Self::manhatten(*sensor, closest_y);

                sensor_to_y <= *sensor_to_beacon
            })
            .map(|(sensor, sensor_to_beacon)| {
                let y_diff = at_y.abs_diff(sensor.1);
                let surrounding_at_y = sensor_to_beacon.abs_diff(y_diff) as isize;

                match clamp {
                    None => ((sensor.0 - surrounding_at_y), (sensor.0 + surrounding_at_y)),
                    Some(x) => (0.max(sensor.0 - surrounding_at_y), x.min(sensor.0 + surrounding_at_y)),
                }
            })
            .collect();

        // Sort for speed since we check from x = 0
        ranges.sort_by_key(|range| range.0);

        ranges
    }

    fn exclude_zone_size_for(&self, y: isize) -> isize {
        let ranges = self.intersecting_x_ranges(y, None);
        let min_x = ranges.iter().map(|range| range.0).min().unwrap();
        let max_x = ranges.iter().map(|range| range.1).max().unwrap();
        max_x - min_x
    }

    fn non_excluded_coord_in_area(&self, area_max: isize) -> (isize, isize) {
        for y in 0..=area_max {
            let intersecting_x_ranges = self.intersecting_x_ranges(y, Some(area_max));

            for (_, end_a) in &intersecting_x_ranges {
                for (start_b, _) in &intersecting_x_ranges {

                    // Candidate x after range a
                    let candidate_x = end_a + 1;

                    // Check if x exists in a gap between range a and range b
                    if *start_b - 1 != candidate_x {
                        continue;
                    }

                    // check if this candidate x is inside the exclusion zone for this y
                    let x_in_exclude_zone = intersecting_x_ranges
                        .iter()
                        .any(|(start, end)| candidate_x >= *start && candidate_x <= *end);

                    if !x_in_exclude_zone {
                        return (candidate_x, y);
                    }
                }
            }
        }

        (0, 0)
    }
}


pub fn day15(input: &str) -> (String, String) {
    let dependant = if cfg!(test) { 10 } else { 2_000_000 };

    let map = Map::parse(input);

    let at_y = dependant;
    let part1 = map.exclude_zone_size_for(at_y).to_string();

    let max_area = dependant * 2;
    let (x, y) = map.non_excluded_coord_in_area(max_area);
    let part2 = (x * 4_000_000 + y).to_string();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    const INPUT_LINES: [&str; 14] = [
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ];

    #[test]
    fn test_day15() {
        let input = INPUT_LINES.join("\n");
        let (part1, part2) = super::day15(&input);

        assert_eq!(part1, "26");
        assert_eq!(part2, "56000011");
    }
}
