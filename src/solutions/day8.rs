#[derive(Debug)]
struct TreeMap {
    width: usize,
    height: usize,
    tree_heights: Vec<usize>,
}

impl TreeMap {
    fn new(input: &str) -> TreeMap {
        let width: usize = input.trim().lines().next().unwrap().len();
        let height: usize = input.trim().lines().count();

        let tree_heights: Vec<usize> = input
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .map(|ch| ch.to_digit(10).unwrap())
            .map(|i| i as usize)
            .collect();

        TreeMap { width, height, tree_heights }
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (y * self.height) + x
    }

    fn idx_to_xy(&self, idx: usize) -> (usize, usize) {
        let y = idx / self.height;
        let x = idx % self.height;
        (x, y)
    }

    fn tree_height_at(&self, x: usize, y: usize) -> &usize {
        self.tree_heights.get(self.xy_to_idx(x, y)).unwrap_or(&0)
    }
}

struct TreeMapSurvey {
    outside_visibilities: Vec<bool>,
    scenic_scores: Vec<usize>,
}

impl TreeMapSurvey {
    fn from_tree_map(tree_map: &TreeMap) -> TreeMapSurvey {
        let mut outside_visibilities: Vec<bool> = Vec::new();
        let mut scenic_scores: Vec<usize> = Vec::new();

        for (idx, tree_height) in tree_map.tree_heights.iter().enumerate() {
            let (visible_up, distance_up) = TreeMapSurvey::survey_trees_up(tree_map, idx, tree_height);
            let (visible_down, distance_down) = TreeMapSurvey::survey_trees_down(tree_map, idx, tree_height);
            let (visible_left, distance_left) = TreeMapSurvey::survey_trees_left(tree_map, idx, tree_height);
            let (visible_right, distance_right) = TreeMapSurvey::survey_trees_right(tree_map, idx, tree_height);

            let visible_outside = visible_up || visible_down || visible_left || visible_right;
            let scenic_score = distance_up * distance_down * distance_left * distance_right;

            outside_visibilities.push(visible_outside);
            scenic_scores.push(scenic_score);
        }

        TreeMapSurvey { outside_visibilities, scenic_scores }
    }

    fn survey_trees_up(tree_map: &TreeMap, idx: usize, tree_height: &usize) -> (bool, usize) {
        let (x, y) = tree_map.idx_to_xy(idx);
        let mut viewing_distance: usize = 0;

        if y == 0 {
            return (true, viewing_distance);
        }

        for test_y in (0..y).rev() {
            viewing_distance += 1;

            if tree_map.tree_height_at(x, test_y) >= tree_height {
                return (false, viewing_distance);
            }
        }

        (true, viewing_distance)
    }

    fn survey_trees_down(tree_map: &TreeMap, idx: usize, tree_height: &usize) -> (bool, usize) {
        let (x, y) = tree_map.idx_to_xy(idx);
        let mut viewing_distance: usize = 0;

        if y == tree_map.height - 1 {
            return (true, viewing_distance);
        }

        for test_y in (y + 1)..tree_map.height {
            viewing_distance += 1;

            if tree_map.tree_height_at(x, test_y) >= tree_height {
                return (false, viewing_distance);
            }
        }

        (true, viewing_distance)
    }

    fn survey_trees_left(tree_map: &TreeMap, idx: usize, tree_height: &usize) -> (bool, usize) {
        let (x, y) = tree_map.idx_to_xy(idx);
        let mut viewing_distance: usize = 0;

        if x == 0 {
            return (true, viewing_distance);
        }

        for test_x in (0..x).rev() {
            viewing_distance += 1;

            if tree_map.tree_height_at(test_x, y) >= tree_height {
                return (false, viewing_distance);
            }
        }

        (true, viewing_distance)
    }

    fn survey_trees_right(tree_map: &TreeMap, idx: usize, tree_height: &usize) -> (bool, usize)  {
        let (x, y) = tree_map.idx_to_xy(idx);
        let mut viewing_distance: usize = 0;

        if x == tree_map.width - 1 {
            return (true, viewing_distance);
        }

        for test_x in (x + 1)..tree_map.width {
            viewing_distance += 1;

            if tree_map.tree_height_at(test_x, y) >= tree_height {
                return (false, viewing_distance);
            }
        }

        (true, viewing_distance)
    }
}

pub fn day8(input: &str) -> (String, String) {
    let tree_map: TreeMap = TreeMap::new(input);
    let tree_map_survey: TreeMapSurvey = TreeMapSurvey::from_tree_map(&tree_map);

    let part1 = tree_map_survey
        .outside_visibilities
        .iter()
        .filter(|tf| **tf)
        .count()
        .to_string();

    let part2 = tree_map_survey
        .scenic_scores
        .iter()
        .max()
        .unwrap()
        .to_string();

    (part1, part2)
}


#[cfg(test)]
mod tests {
    const INPUT: &str = "
        30373
        25512
        65332
        33549
        35390";

    #[test]
    fn test_day8() {
        let (part1, part2) = super::day8(INPUT);

        assert_eq!(part1, "21");
        assert_eq!(part2, "8");
    }
}
