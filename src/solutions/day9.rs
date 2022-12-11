use std::collections::HashSet;

pub fn day9(input: &str) -> (String, String) {
    let instructions: Vec<(char, isize)> = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| (parts[0].chars().next().unwrap(), parts[1].parse::<isize>().unwrap()))
        .collect();

    let head_positions = head_positions(instructions);
    let tail_positions_1 = tail_positions(&head_positions);
    let tail_positions_2 = tail_positions(&tail_positions_1);
    let tail_positions_3 = tail_positions(&tail_positions_2);
    let tail_positions_4 = tail_positions(&tail_positions_3);
    let tail_positions_5 = tail_positions(&tail_positions_4);
    let tail_positions_6 = tail_positions(&tail_positions_5);
    let tail_positions_7 = tail_positions(&tail_positions_6);
    let tail_positions_8 = tail_positions(&tail_positions_7);
    let tail_positions_9 = tail_positions(&tail_positions_8);

    let uniq_tail_positions_1: HashSet<(isize, isize)> = HashSet::from_iter(tail_positions_1.into_iter());
    let uniq_tail_positions_9: HashSet<(isize, isize)> = HashSet::from_iter(tail_positions_9.into_iter());
    let part1 = uniq_tail_positions_1.len().to_string();
    let part2 = uniq_tail_positions_9.len().to_string();

    (part1, part2)
}

fn tail_positions(head_positions: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut tail_positions: Vec<(isize, isize)> = Vec::new();

    tail_positions.push((0, 0));

    for head_pos in head_positions {
        let (head_x, head_y) = head_pos;
        let &tail_pos = tail_positions.last().unwrap();
        let (tail_x, tail_y) = tail_pos;

        let off_x = head_x - tail_x;
        let off_y = head_y - tail_y;

        if off_x.abs() <= 1 && off_y.abs() <= 1 {
            continue;
        }

        // Part 1
        // ...HHH.....
        // ..H...H....
        // .H..T..H...
        // ..H...H....
        // ...HHH.....

        // Part 2
        // ..H...H....
        // ...........
        // ....T......
        // ...........
        // ..H...H....
        let new_tail_pos = match (off_x, off_y) {
            // Part 1
            ( 0,  2) => (tail_x    , tail_y + 1),
            ( 0, -2) => (tail_x    , tail_y - 1),

            ( 2,  0) => (tail_x + 1, tail_y    ),
            (-2,  0) => (tail_x - 1, tail_y    ),

            ( 2,  1) => (tail_x + 1, tail_y + 1),
            ( 1,  2) => (tail_x + 1, tail_y + 1),

            (-2,  1) => (tail_x - 1, tail_y + 1),
            (-1,  2) => (tail_x - 1, tail_y + 1),

            (-2, -1) => (tail_x - 1, tail_y - 1),
            (-1, -2) => (tail_x - 1, tail_y - 1),

            ( 2, -1) => (tail_x + 1, tail_y - 1),
            ( 1, -2) => (tail_x + 1, tail_y - 1),

            // Part 2
            ( 2,  2) => (tail_x + 1, tail_y + 1),
            (-2,  2) => (tail_x - 1, tail_y + 1),
            ( 2, -2) => (tail_x + 1, tail_y - 1),
            (-2, -2) => (tail_x - 1, tail_y - 1),
            _ => panic!(),
        };

        tail_positions.push(new_tail_pos);
    }

    tail_positions
}

fn head_positions(instructions: Vec<(char, isize)>) -> Vec<(isize, isize)> {
    let mut head_positions: Vec<(isize, isize)> = Vec::new();

    head_positions.push((0, 0));

    for (dir, dist) in instructions {
        let &(x, y) = head_positions.last().unwrap();

        let mut movements: Vec<(isize, isize)> = match dir {
            'U' => ((y + 1)..=(y + dist)).map(|y| (x, y)).collect(),
            'D' => ((y - dist)..y).rev().map(|y| (x, y)).collect(),
            'R' => ((x + 1)..=(x + dist)).map(|x| (x, y)).collect(),
            'L' => ((x - dist)..x).rev().map(|x| (x, y)).collect(),
            _ => panic!()
        };

        head_positions.append(&mut movements);
    }

    head_positions
}

#[cfg(test)]
mod tests {
    const INPUTS: [&str; 2] = [
    "R 4
     U 4
     L 3
     D 1
     R 4
     D 1
     L 5
     R 2",

     "R 5
      U 8
      L 8
      D 3
      R 17
      D 10
      L 25
      U 20",
    ];

    #[test]
    fn test_day9() {
        let (part1, part2) = super::day9(INPUTS[0]);

        assert_eq!(part1, "13");
        assert_eq!(part2, "1");

        let (part1, part2) = super::day9(INPUTS[1]);

        assert_eq!(part1, "88");
        assert_eq!(part2, "36");
    }
}
