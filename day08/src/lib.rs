pub struct Forest {
    width: usize,
    height: usize,
    grid: Vec<u32>,
}

impl Forest {
    pub fn from_string(input: &str) -> Self {
        let width = input.chars().position(|x| x == '\n').unwrap();
        let grid : Vec<u32> = input
            .chars()
            .filter(|x| *x != '\n')
            .flat_map(|tree| tree.to_digit(10))
            .collect();
        let height = grid.len() / width;
        Forest { width, grid, height}
    }

    pub fn number_of_visibles(&self) -> usize {
        (0..self.grid.len())
            .filter(|idx| self.is_visible(*idx))
            .count()
    }

    pub fn highest_scenic_score(&self) -> usize {
        (0..self.grid.len())
            .map(|idx| self.scenic_score(idx))
            .max()
            .unwrap()
    }

    fn idx_to_coords(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn coords_to_idx(&self, i: usize, j: usize) -> usize {
        i + j * self.width
    }

    fn get(&self, i: usize, j: usize) -> u32 {
        self.grid[self.coords_to_idx(i, j)]
    }

    fn is_visible(&self, idx: usize) -> bool {
        let (i, j) = self.idx_to_coords(idx);
        self.is_visible_from_left(i, j)
            || self.is_visible_from_right(i, j)
            || self.is_visible_from_top(i, j)
            || self.is_visible_from_bottom(i, j)
    }

    fn scenic_score(&self, idx: usize) -> usize {
        let (i, j) = self.idx_to_coords(idx);
        self.viewing_distance_left(i, j)
            * self.viewing_distance_right(i, j)
            * self.viewing_distance_top(i, j)
            * self.viewing_distance_bottom(i, j)
    }


    fn is_visible_from_left(&self, mut i: usize, j: usize) -> bool {
        let tree = self.get(i, j);

        while i > 0 {
            if self.get(i - 1, j) >= tree {
                return false;
            }
            i -= 1;
        }
        true
    }

    fn is_visible_from_right(&self, mut i: usize, j: usize) -> bool {
        let tree = self.get(i, j);

        while i < self.width - 1 {
            if self.get(i + 1, j) >= tree {
                return false;
            }
            i += 1;
        }
        true
    }

    fn is_visible_from_top(&self, i: usize, mut j: usize) -> bool {
        let tree = self.get(i, j);

        while j > 0 {
            if self.get(i, j - 1)>= tree {
                return false;
            }
            j -= 1;
        }
        true
    }

    fn is_visible_from_bottom(&self, i: usize, mut j: usize) -> bool {
        let tree = self.get(i, j);

        while j < self.height - 1 {
            if self.get(i, j + 1) >= tree {
                return false;
            }
            j += 1;
        }
        true
    }

    fn viewing_distance_left(&self, mut i: usize, j: usize) -> usize {
        let tree = self.get(i, j);

        let mut result = 0;
        while i > 0 {
            result += 1;
            if self.get(i - 1, j) >= tree {
                break;
            }
            i -= 1;
        }
        result
    }

    fn viewing_distance_right(&self, mut i: usize, j: usize) -> usize {
        let tree = self.get(i, j);

        let mut result = 0;
        while i < self.width - 1 {
            result += 1;
            if self.get(i + 1, j) >= tree {
                break;
            }
            i += 1;
        }
        result
    }

    fn viewing_distance_top(&self, i: usize, mut j: usize) -> usize {
        let tree = self.get(i, j);

        let mut result = 0;
        while j > 0 {
            result += 1;
            if self.get(i, j - 1) >= tree {
                break;
            }
            j -= 1;
        }
        result
    }

    fn viewing_distance_bottom(&self, i: usize, mut j: usize) -> usize {
        let tree = self.get(i, j);

        let mut result = 0;
        while j < self.height - 1 {
            result += 1;
            if self.get(i, j + 1) >= tree {
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
