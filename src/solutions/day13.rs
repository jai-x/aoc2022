use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

trait StringExt {
    fn is_bracketed(&self) -> bool;
    fn strip_brackets(self) -> String;
    fn chunk_outer(self) -> Vec<String>;
}

impl StringExt for String {
    fn is_bracketed(&self) -> bool {
        if self.len() < 2 {
            return false;
        }

        self.starts_with('[') && self.ends_with(']')
    }

    fn strip_brackets(self) -> String {
        if !self.is_bracketed() {
            return self;
        }

        if self == "[]" {
            return String::new();
        }

        let mut chars = self.chars();
        chars.next();
        chars.next_back();
        chars.as_str().into()
    }

    fn chunk_outer(self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();

        if self.is_empty() {
            return out;
        }

        let mut depth: usize = 0;
        let mut comma_idxs: Vec<usize> = Vec::new();

        for (i, ch) in self.chars().enumerate() {
            match ch {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {
                    if depth == 0 {
                        comma_idxs.push(i);
                    }
                }
                _ => (),
            }
        }

        if comma_idxs.is_empty() {
            out.push(self);
            return out;
        }

        self.chars()
            .enumerate()
            .map(|(i, ch)| if comma_idxs.contains(&i) { ' ' } else { ch })
            .collect::<String>()
            .split_whitespace()
            .map(String::from)
            .collect()
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Num(usize),
    List(Vec<Packet>),
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Num(num) => write!(f, "{}", num),
            Self::List(list) => {
                let formatted = list
                    .iter()
                    .map(|packet| format!("{:?}", packet))
                    .collect::<Vec<_>>()
                    .join(",");

                write!(f, "[{}]", formatted)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::{List, Num};

        match (self, other) {
            // Wrap number as list
            (lhs_num @ Num(_), rhs_list @ List(_)) => {
                let lhs_list = List(vec![lhs_num.clone()]);
                lhs_list.cmp(rhs_list)
            }

            // Wrap number as list
            (lhs_list @ List(_), rhs_num @ Num(_)) => {
                let rhs_list = List(vec![rhs_num.clone()]);
                lhs_list.cmp(&rhs_list)
            }

            // Compare numbers
            (Num(lhs_num), Num(rhs_num)) => lhs_num.cmp(rhs_num),

            // Compare lists
            (List(lhs_list), List(rhs_list)) => {
                let (lhs_len, rhs_len) = (lhs_list.len(), rhs_list.len());
                let min_len = lhs_len.min(rhs_len);

                for i in 0..min_len {
                    let (l_elem, r_elem) = (&lhs_list[i], &rhs_list[i]);

                    match l_elem.cmp(r_elem) {
                        Ordering::Equal => continue,
                        order => return order,
                    }
                }

                // One or both lists have run out
                lhs_len.cmp(&rhs_len)
            }
        }
    }
}

impl Packet {
    fn parse(input: String) -> Packet {
        use Packet::{List, Num};

        if !input.is_bracketed() {
            return Num(input.trim().parse().unwrap());
        }

        let inner = input.strip_brackets();

        let mut out: Vec<Self> = Vec::new();

        for chunk in inner.chunk_outer() {
            out.push(Self::parse(chunk));
        }

        List(out)
    }
}

pub fn day13(input: &str) -> (String, String) {
    let mut packets: Vec<Packet> = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::parse(line.to_string()))
        .collect();

    let part1 = packets
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] < pair[1])
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string();

    // Insert divider packets and sort
    packets.push(Packet::parse("[[2]]".to_string()));
    packets.push(Packet::parse("[[6]]".to_string()));
    packets.sort();

    // Construct new dividers to compare to since previous ones were moved
    let divider1 = Packet::parse("[[2]]".to_string());
    let divider2 = Packet::parse("[[6]]".to_string());

    let part2 = packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| *packet == divider1 || *packet == divider2)
        .map(|(i, _)| i + 1)
        .product::<usize>()
        .to_string();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    const INPUT_LINES: [&str; 23] = [
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ];

    #[test]
    fn test_day13() {
        use super::day13;

        let input = INPUT_LINES.join("\n");
        let (part1, part2) = day13(&input);

        assert_eq!(part1, "13");
        assert_eq!(part2, "140");
    }

    #[test]
    fn test_day13_packet_parse() {
        use super::Packet;
        use super::Packet::{List, Num};

        let parsed = Packet::parse("[1,1,3,1,1]".to_string());
        let packet = List(vec![Num(1), Num(1), Num(3), Num(1), Num(1)]);
        assert_eq!(parsed, packet);

        let parsed = Packet::parse("[[[]]]".to_string());
        let packet = List(vec![List(vec![List(vec![])])]);
        assert_eq!(parsed, packet);

        let parsed = Packet::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string());
        let packet = List(vec![
            Num(1),
            List(vec![
                Num(2),
                List(vec![
                    Num(3),
                    List(vec![Num(4), List(vec![Num(5), Num(6), Num(7)])]),
                ]),
            ]),
            Num(8),
            Num(9),
        ]);
        assert_eq!(parsed, packet);
    }

    #[test]
    fn test_day13_packet_ord() {
        use super::Packet::{List, Num};

        let lhs = List(vec![Num(1), Num(1), Num(3), Num(1), Num(1)]);
        let rhs = List(vec![Num(1), Num(1), Num(5), Num(1), Num(1)]);
        assert_eq!(lhs < rhs, true);

        let lhs = List(vec![List(vec![Num(1)]), List(vec![Num(2), Num(3), Num(4)])]);
        let rhs = List(vec![List(vec![Num(1)]), Num(4)]);
        assert_eq!(lhs < rhs, true);

        let lhs = List(vec![Num(9)]);
        let rhs = List(vec![List(vec![Num(8), Num(7), Num(6)])]);
        assert_eq!(lhs < rhs, false);
    }

    #[test]
    fn test_day13_string_is_bracketed() {
        use super::StringExt;

        let string = String::from("i");
        assert_eq!(string.is_bracketed(), false);

        let string = String::from("[[");
        assert_eq!(string.is_bracketed(), false);

        let string = String::from("]]");
        assert_eq!(string.is_bracketed(), false);

        let string = String::from("[]");
        assert_eq!(string.is_bracketed(), true);

        let string = String::from("[1234]");
        assert_eq!(string.is_bracketed(), true);
    }

    #[test]
    fn test_day13_string_strip_brackets() {
        use super::StringExt;

        let string = String::from("i");
        assert_eq!(string.strip_brackets(), "i");

        let string = String::from("[[");
        assert_eq!(string.strip_brackets(), "[[");

        let string = String::from("]]");
        assert_eq!(string.strip_brackets(), "]]");

        let string = String::from("[]");
        assert_eq!(string.strip_brackets(), "");

        let string = String::from("[1234]");
        assert_eq!(string.strip_brackets(), "1234");
    }

    #[test]
    fn test_day13_string_chunk_outer() {
        use super::StringExt;

        let string = String::from("");
        assert_eq!(string.chunk_outer(), Vec::<String>::new());

        let string = String::from("0");
        assert_eq!(string.chunk_outer(), vec!["0"]);

        let string = String::from("0,1");
        assert_eq!(string.chunk_outer(), vec!["0", "1"]);

        let string = String::from("[1],[2,3,4]");
        assert_eq!(string.chunk_outer(), vec!["[1]", "[2,3,4]"]);

        let string = String::from("[4,4],4,4");
        assert_eq!(string.chunk_outer(), vec!["[4,4]", "4", "4"]);
    }
}
