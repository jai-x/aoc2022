use std::collections::VecDeque;

#[derive(Debug)]
struct Monkies {
    items: Vec<VecDeque<usize>>,
    op_operators: Vec<char>,
    op_values: Vec<Option<usize>>,
    div_tests: Vec<usize>,
    true_indexes: Vec<usize>,
    false_indexes: Vec<usize>,
    businesses: Vec<usize>,
    gcd: usize, // Assume all div check number are prime so gcd is just product
}

impl Monkies {
    fn parse(input: &str) -> Monkies {
        let mut items: Vec<VecDeque<usize>> = Vec::new();
        let mut op_operators: Vec<char> = Vec::new();
        let mut op_values: Vec<Option<usize>> = Vec::new();
        let mut div_tests: Vec<usize> = Vec::new();
        let mut true_indexes: Vec<usize> = Vec::new();
        let mut false_indexes: Vec<usize> = Vec::new();
        let mut businesses: Vec<usize> = Vec::new();

        let monkey_inputs: Vec<Vec<&str>> = input
            .split("\n\n")
            .map(|group| group.lines().collect::<Vec<&str>>())
            .map(|lines| {
                lines
                    .into_iter()
                    .map(|line| line.trim())
                    .collect::<Vec<&str>>()
            })
            .collect();

        for monkey_lines in monkey_inputs {
            let starting_items: VecDeque<usize> = monkey_lines[1]
                .trim_start_matches("Starting items: ")
                .split(',')
                .filter_map(|piece| piece.trim().parse().ok())
                .collect();

            let operator: char = monkey_lines[2]
                .trim_start_matches("Operation: new = old ")
                .split_whitespace()
                .next()
                .unwrap()
                .chars()
                .next()
                .unwrap();

            let value: Option<usize> = monkey_lines[2]
                .trim_start_matches("Operation: new = old ")
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .ok();

            let div_test: usize = monkey_lines[3]
                .trim_start_matches("Test: divisible by ")
                .parse()
                .unwrap();

            let monkey_true: usize = monkey_lines[4]
                .trim_start_matches("If true: throw to monkey ")
                .parse()
                .unwrap();

            let monkey_false: usize = monkey_lines[5]
                .trim_start_matches("If false: throw to monkey ")
                .parse()
                .unwrap();

            items.push(starting_items);
            op_operators.push(operator);
            op_values.push(value);
            div_tests.push(div_test);
            true_indexes.push(monkey_true);
            false_indexes.push(monkey_false);
            businesses.push(0);
        }

        let gcd = div_tests.iter().product();

        Monkies {
            items,
            op_operators,
            op_values,
            div_tests,
            true_indexes,
            false_indexes,
            businesses,
            gcd,
        }
    }

    fn do_round(&mut self, worry_divide: usize) {
        for i in 0..self.items.len() {
            while let Some(old_worry) = self.items[i].pop_front() {
                let op_value = match self.op_values[i] {
                    Some(val) => val,
                    None => old_worry,
                };

                let inspect_worry = match self.op_operators[i] {
                    '*' => old_worry * op_value,
                    '+' => old_worry + op_value,
                    opr => panic!("Unknown opr: {}", opr),
                };

                let bored_worry = inspect_worry / worry_divide;

                // Modular arithmetic yo!
                let simplified_worry = bored_worry % self.gcd;

                let throw_idx = match simplified_worry % self.div_tests[i] {
                    0 => self.true_indexes[i],
                    _ => self.false_indexes[i],
                };

                self.businesses[i] += 1;
                self.items[throw_idx].push_back(simplified_worry);
            }
        }
    }

    fn business(&self) -> usize {
        let mut b = self.businesses.clone();
        b.sort();
        b.into_iter().rev().take(2).product::<usize>()
    }
}

pub fn day11(input: &str) -> (String, String) {
    let mut p1_monkies = Monkies::parse(input);
    for _ in 0..20 {
        p1_monkies.do_round(3);
    }
    let part1 = p1_monkies.business().to_string();

    let mut p2_monkies = Monkies::parse(input);
    for _ in 0..10_000 {
        p2_monkies.do_round(1);
    }
    let part2 = p2_monkies.business().to_string();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    const INPUT_LINES: [&str; 27] = [
        "Monkey 0:",
        "  Starting items: 79, 98",
        "  Operation: new = old * 19",
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
        "",
        "Monkey 1:",
        "  Starting items: 54, 65, 75, 74",
        "  Operation: new = old + 6",
        "  Test: divisible by 19",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 0",
        "",
        "Monkey 2:",
        "  Starting items: 79, 60, 97",
        "  Operation: new = old * old",
        "  Test: divisible by 13",
        "    If true: throw to monkey 1",
        "    If false: throw to monkey 3",
        "",
        "Monkey 3:",
        "  Starting items: 74",
        "  Operation: new = old + 3",
        "  Test: divisible by 17",
        "    If true: throw to monkey 0",
        "    If false: throw to monkey 1",
    ];

    #[test]
    fn test_day11() {
        let input = INPUT_LINES.join("\n");
        let (part1, part2) = super::day11(&input);

        assert_eq!(part1, "10605");
        assert_eq!(part2, "2713310158");
    }
}
