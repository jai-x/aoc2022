pub fn day5(input: &str) -> (String, String) {
    let (stack_instr, move_instr) = input.split_once("\n\n").unwrap();

    let stacks = build_stacks(stack_instr);
    let movement = build_movement(move_instr);

    let part1 = cratemover_9000(stacks.clone(), &movement);
    let part2 = cratemover_9001(stacks, &movement);

    (part1, part2)
}

fn cratemover_9001(mut stacks: Vec<Vec<char>>, movement: &[(usize, usize, usize)]) -> String {
    let mut copy_stack = Vec::new();
    for &(amount, source, dest) in movement {
        for _ in 0..amount {
            copy_stack.push(stacks[source].pop().unwrap());
        }
        copy_stack.reverse();
        stacks[dest].append(&mut copy_stack);
    }
    top_elements(stacks)
}

fn cratemover_9000(mut stacks: Vec<Vec<char>>, movement: &[(usize, usize, usize)]) -> String {
    for &(amount, source, dest) in movement {
        for _ in 0..amount {
            let ch = stacks[source].pop().unwrap();
            stacks[dest].push(ch);
        }
    }

    top_elements(stacks)
}

fn top_elements(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>()
}

fn build_movement(move_instructions: &str) -> Vec<(usize, usize, usize)> {
    move_instructions
        .lines()
        .map(|line| line.chars().map(|ch| match ch { '0'..='9' => ch, _ => ' ', }))
        .map(|chars| chars.collect::<String>())
        .map(|num_only| {
            num_only
                .split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|vec| (vec[0], vec[1] - 1, vec[2] - 1))
        .collect()
}

fn build_stacks(stack_instructions: &str) -> Vec<Vec<char>> {
    let lines = stack_instructions.lines().collect::<Vec<&str>>();
    let stack_count = lines.last().unwrap().split_whitespace().count();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];

    for line in lines.into_iter().rev().skip(1) {
        for (i, ch) in line.chars().enumerate() {
            if ('A'..='Z').contains(&ch) {
                let idx = i / 4;
                stacks[idx].push(ch);
            }
        }
    }

    stacks
}

#[cfg(test)]
mod tests {
    // curse whitespace dependant input
    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_day5() {
        let (part1, part2) = super::day5(INPUT);

        assert_eq!(part1, "CMZ");
        assert_eq!(part2, "MCD");
    }
}
