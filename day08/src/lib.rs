pub struct Forest(Vec<Vec<u32>>);

impl Forest {
    pub fn from_string(input: &str) -> Self {
        Forest(
            input
                .lines()
                .map(|line| line.chars().flat_map(|tree| tree.to_digit(10)).collect())
                .collect(),
        )
    }

    pub fn number_of_visibles(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(j, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(i, _)| self.is_visible(*i, j))
                    .count()
            })
            .sum()
    }

    pub fn highest_scenic_score(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(j, row)| {
                row.iter()
                    .enumerate()
                    .map(|(i, _)| self.scenic_score(i, j))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        self.is_visible_from_left(row, col)
            || self.is_visible_from_right(row, col)
            || self.is_visible_from_top(row, col)
            || self.is_visible_from_bottom(row, col)
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        self.viewing_distance_left(row, col)
            * self.viewing_distance_right(row, col)
            * self.viewing_distance_top(row, col)
            * self.viewing_distance_bottom(row, col)
    }

    fn is_visible_from_left(&self, row: usize, col: usize) -> bool {
        let tree = self.0[col][row];

        let mut i = row;
        while i > 0 {
            if self.0[col][i - 1] >= tree {
                return false;
            }
            i -= 1;
        }
        true
    }

    fn is_visible_from_right(&self, row: usize, col: usize) -> bool {
        let tree = self.0[col][row];

        let mut i = row;
        while i < self.0.len() - 1 {
            if self.0[col][i + 1] >= tree {
                return false;
            }
            i += 1;
        }
        true
    }

    fn is_visible_from_top(&self, row: usize, col: usize) -> bool {
        let tree = self.0[col][row];

        let mut j = col;
        while j > 0 {
            if self.0[j - 1][row] >= tree {
                return false;
            }
            j -= 1;
        }
        true
    }

    fn is_visible_from_bottom(&self, row: usize, col: usize) -> bool {
        let tree = self.0[col][row];

        let mut j = col;
        while j < self.0.len() - 1 {
            if self.0[j + 1][row] >= tree {
                return false;
            }
            j += 1;
        }
        true
    }

    fn viewing_distance_left(&self, row: usize, col: usize) -> usize {
        let tree = self.0[col][row];

        let mut result = 0;
        let mut i = row;
        while i > 0 {
            result += 1;
            if self.0[col][i - 1] >= tree {
                break;
            }
            i -= 1;
        }
        result
    }

    fn viewing_distance_right(&self, row: usize, col: usize) -> usize {
        let tree = self.0[col][row];

        let mut result = 0;
        let mut i = row;
        while i < self.0.len() - 1 {
            result += 1;
            if self.0[col][i + 1] >= tree {
                break;
            }
            i += 1;
        }
        result
    }

    fn viewing_distance_top(&self, row: usize, col: usize) -> usize {
        let tree = self.0[col][row];

        let mut result = 0;
        let mut j = col;
        while j > 0 {
            result += 1;
            if self.0[j - 1][row] >= tree {
                break;
            }
            j -= 1;
        }
        result
    }

    fn viewing_distance_bottom(&self, row: usize, col: usize) -> usize {
        let tree = self.0[col][row];

        let mut result = 0;
        let mut j = col;
        while j < self.0.len() - 1 {
            result += 1;
            if self.0[j + 1][row] >= tree {
                break;
            }
            j += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn forest_number_of_visibles() {
        // GIVEN
        let input = indoc! {"
            30373
            25512
            65332
            33549
            35390
        "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.number_of_visibles();

        // THEN
        assert_eq!(21, num);
    }

    #[test]
    fn forest_viewing_distance_left() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.viewing_distance_left(2, 1);

        // THEN
        assert_eq!(1, num);
    }

    #[test]
    fn forest_viewing_distance_left2() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.viewing_distance_left(2, 3);

        // THEN
        assert_eq!(2, num);
    }

    #[test]
    fn forest_viewing_distance_right() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.viewing_distance_right(2, 1);

        // THEN
        assert_eq!(2, num);
    }

    #[test]
    fn forest_viewing_distance_right2() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.viewing_distance_right(2, 3);

        // THEN
        assert_eq!(2, num);
    }

    #[test]
    fn forest_viewing_distance_top() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.viewing_distance_top(2, 1);

        // THEN
        assert_eq!(1, num);
    }

    #[test]
    fn forest_viewing_distance_top2() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.viewing_distance_top(2, 3);

        // THEN
        assert_eq!(2, num);
    }

    #[test]
    fn forest_viewing_distance_bottom() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.viewing_distance_bottom(2, 1);

        // THEN
        assert_eq!(2, num);
    }

    #[test]
    fn forest_viewing_distance_bottom2() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.viewing_distance_bottom(2, 3);

        // THEN
        assert_eq!(1, num);
    }

    #[test]
    fn forest_highest_scenic_score() {
        // GIVEN
        let input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

        let forest = Forest::from_string(input);

        // WHEN
        let num = forest.highest_scenic_score();

        // THEN
        assert_eq!(8, num);
    }
}
