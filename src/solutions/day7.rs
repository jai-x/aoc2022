use std::collections::HashMap;

pub fn day7(input: &str) -> (String, String) {
    let commands: Vec<Vec<&str>> = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let ordered_dir_sizes: Vec<usize> = ordered_dir_sizes(commands);

    let threshold_size: usize = 100_000;
    let total_space: usize = 70_000_000;
    let needed_free_space: usize = 30_000_000;

    let part1 = ordered_dir_sizes
        .iter()
        .filter(|&&size| size < threshold_size)
        .sum::<usize>()
        .to_string();

    let current_free_space = total_space - ordered_dir_sizes.last().unwrap();

    let part2 = ordered_dir_sizes
        .iter()
        .find(|&&size| current_free_space + size >= needed_free_space)
        .unwrap()
        .to_string();

    (part1, part2)
}

fn ordered_dir_sizes(commands: Vec<Vec<&str>>) -> Vec<usize> {
    let mut pwd_stack: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for line in commands {
        match line.as_slice() {
            ["$", "ls"] => {
                continue;
            }

            ["dir", _] => {
                continue;
            }

            ["$", "cd", ".."] => {
                pwd_stack.pop();
                continue;
            }

            ["$", "cd", "/"] => {
                pwd_stack.clear();
                pwd_stack.push("root".to_string());
                continue;
            }

            ["$", "cd", folder] => {
                pwd_stack.push(folder.to_string());
                continue;
            }

            [file_size, _] => {
                let file_size = file_size.parse::<usize>().unwrap();

                for i in 0..pwd_stack.len() {
                    let folder_path = pwd_stack[..=i].join("/");

                    dir_sizes
                        .entry(folder_path.to_string())
                        .and_modify(|size| *size += file_size)
                        .or_insert(file_size);
                }

                continue;
            }

            _ => (),
        };
    }

    let mut ordered_dir_sizes: Vec<usize> = dir_sizes.values().cloned().collect();
    ordered_dir_sizes.sort();

    ordered_dir_sizes
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";

    #[test]
    fn test_day7() {
        let (part1, part2) = super::day7(INPUT);

        assert_eq!(part1, "95437");
        assert_eq!(part2, "24933642");
    }
}
