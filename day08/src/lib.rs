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
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(j, tree)| self.is_visible(**tree, i, *j))
                    .count()
            })
            .sum()
    }

    fn is_visible(&self, tree: u32, row: usize, col: usize) -> bool {
        self.is_visible_from_left(tree, row, col)
            || self.is_visible_from_right(tree, row, col)
            || self.is_visible_from_top(tree, row, col)
            || self.is_visible_from_bottom(tree, row, col)
    }

    fn is_visible_from_left(&self, tree: u32, row: usize, col: usize) -> bool {
        if row == 0 {
            return true;
        }
        let mut i = row;
        while i > 0 {
            if self.0[i - 1][col] >= tree {
                return false;
            }
            i -= 1;
        }
        true
    }

    fn is_visible_from_right(&self, tree: u32, row: usize, col: usize) -> bool {
        if row == self.0.len() - 1 {
            return true;
        }
        let mut i = row;
        while i < self.0.len() - 1 {
            if self.0[i + 1][col] >= tree {
                return false;
            }
            i += 1;
        }
        true
    }

    fn is_visible_from_top(&self, tree: u32, row: usize, col: usize) -> bool {
        if col == 0 {
            return true;
        }
        let mut j = col;
        while j > 0 {
            if self.0[row][j - 1] >= tree {
                return false;
            }
            j -= 1;
        }
        true
    }

    fn is_visible_from_bottom(&self, tree: u32, row: usize, col: usize) -> bool {
        if col == self.0.len() - 1 {
            return true;
        }
        let mut j = col;
        while j < self.0.len() - 1 {
            if self.0[row][j + 1] >= tree {
                return false;
            }
            j += 1;
        }
        true
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
}
