pub struct Area {
    grid: Vec<u8>,
    width: usize,
    height: usize,
    start: usize,
    end: usize,
    min_steps: Vec<usize>,
}

impl Area {
    pub fn from_string(input: &str) -> Self {
        let width = input.chars().position(|x| x == '\n').unwrap();
        let mut grid: Vec<u8> = input
            .as_bytes()
            .iter()
            .copied()
            .filter(|c| *c != b'\n')
            .collect();
        let height = grid.len() / width;
        let start = grid.iter().position(|x| *x == b'S').unwrap();
        let end = grid.iter().position(|x| *x == b'E').unwrap();
        grid[start] = b'a';
        grid[end] = b'z';
        let mut min_steps = vec![usize::MAX; width * height];
        min_steps[end] = 0;
        let mut area = Area {
            grid,
            width,
            height,
            start,
            end,
            min_steps,
        };
        area.propagate(end);
        area
    }

    pub fn min_steps(&mut self) -> usize {
        self.min_steps[self.start]
    }

    pub fn min_steps_from_all_a(&mut self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, height)| **height == b'a')
            .map(|(i, _)| i)
            .map(|i| self.min_steps[i])
            .min()
            .unwrap()
    }

    fn propagate(&mut self, i: usize) {
        for neigh in self.get_neighbors(i).iter().copied() {
            if self.can_reach(self.grid[neigh], self.grid[i])
                && self.min_steps[i].saturating_add(1) < self.min_steps[neigh]
            {
                self.min_steps[neigh] = self.min_steps[i].saturating_add(1);
                self.propagate(neigh);
            }
        }
    }

    fn can_reach(&self, src: u8, dest: u8) -> bool {
        src + 1 >= dest
    }

    fn get_neighbors(&self, i: usize) -> Vec<usize> {
        let (x, y) = self.idx_to_coords(i);
        let mut neighbors = vec![];

        if x > 0 {
            neighbors.push(self.coords_to_idx(x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push(self.coords_to_idx(x + 1, y));
        }
        if y > 0 {
            neighbors.push(self.coords_to_idx(x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push(self.coords_to_idx(x, y + 1));
        }

        neighbors
    }

    fn idx_to_coords(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn coords_to_idx(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_area() {
        // GIVEN
        let input = indoc! {"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
            "};
        // WHEN
        let area = Area::from_string(input);

        // THEN
        assert_eq!(0, area.start);
        assert_eq!(21, area.end);
    }

    #[test]
    fn min_steps() {
        // GIVEN
        let input = indoc! {"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
            "};

        let mut area = Area::from_string(input);

        // WHEN
        let result = area.min_steps();

        // THEN
        assert_eq!(31, result);
    }

    #[test]
    fn min_steps_from_all_a() {
        // GIVEN
        let input = indoc! {"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
            "};

        let mut area = Area::from_string(input);

        // WHEN
        let result = area.min_steps_from_all_a();

        // THEN
        assert_eq!(29, result);
    }
}
