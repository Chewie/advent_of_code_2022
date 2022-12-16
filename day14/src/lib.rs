use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use nom::{
    bytes::complete::tag,
    character::complete::{char, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

struct Path {
    points: Vec<(u32, u32)>,
}

impl Path {
    fn parse(input: &str) -> IResult<&str, Path> {
        map(
            separated_list1(tag(" -> "), separated_pair(u32, char(','), u32)),
            |points| Path { points },
        )(input)
    }

    fn points_in_pair(src: (u32, u32), dest: (u32, u32)) -> Vec<(u32, u32)> {
        use std::cmp::{max, min};

        if src.0 == dest.0 {
            (min(src.1, dest.1)..=max(src.1, dest.1))
                .map(|j| (src.0, j))
                .collect()
        } else {
            (min(src.0, dest.0)..=max(src.0, dest.0))
                .map(|i| (i, src.1))
                .collect()
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Cell {
    Air,
    Rock,
    Sand,
    Start,
}

impl Cell {
    fn is_blocked(&self) -> bool {
        *self == Cell::Rock || *self == Cell::Sand
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Air => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
            Cell::Start => '+',
        };
        write!(f, "{}", c)
    }
}

pub struct Cave {
    grid: Vec<Cell>,
    current_sand: Option<(u32, u32)>,
    number_of_rests: usize,
    abyss_reached: bool,
    source_reached: bool,
    floor: u32,
}

impl Index<(u32, u32)> for Cave {
    type Output = Cell;

    fn index(&self, (i, j): (u32, u32)) -> &Self::Output {
        &self.grid[i as usize + j as usize * Cave::WIDTH]
    }
}

impl IndexMut<(u32, u32)> for Cave {
    fn index_mut(&mut self, (i, j): (u32, u32)) -> &mut Self::Output {
        &mut self.grid[i as usize + j as usize * Cave::WIDTH]
    }
}

impl Cave {
    const WIDTH: usize = 600;
    const HEIGHT: usize = 500;

    pub fn parse(input: &str) -> IResult<&str, Cave> {
        map(many1(terminated(Path::parse, char('\n'))), |paths| {
            Cave::from_paths(&paths)
        })(input)
    }

    fn new() -> Cave {
        let mut grid = vec![Cell::Air; Cave::WIDTH as usize * Cave::HEIGHT as usize];
        grid[500] = Cell::Start;
        Cave {
            grid,
            current_sand: None,
            number_of_rests: 0,
            abyss_reached: false,
            source_reached: false,
            floor: 0,
        }
    }

    fn from_paths(paths: &[Path]) -> Cave {
        let mut cave = Cave::new();
        for path in paths {
            for window in path.points.windows(2) {
                let (src, dest) = (window[0], window[1]);
                for point in Path::points_in_pair(src, dest) {
                    cave[point] = Cell::Rock;
                    if point.1 + 2 > cave.floor {
                        cave.floor = point.1 + 2;
                    }
                }
            }
        }
        cave
    }

    pub fn number_of_rests(&self) -> usize {
        self.number_of_rests
    }

    // ABYSS VERSION

    pub fn step_abyss(&mut self) {
        match self.current_sand {
            None => {
                self.current_sand = Some((500, 1));
                self[(500, 1)] = Cell::Sand;
            }
            Some((s_x, s_y)) => {
                if !self.try_cell_abyss((s_x, s_y + 1))
                    && !self.try_cell_abyss((s_x - 1, s_y + 1))
                    && !self.try_cell_abyss((s_x + 1, s_y + 1))
                {
                    self.current_sand = None;
                    self.number_of_rests += 1;
                }
                if s_y as usize == Cave::HEIGHT - 2 {
                    self.abyss_reached = true
                }
            }
        }
    }

    fn try_cell_abyss(&mut self, cell: (u32, u32)) -> bool {
        let current_sand = self.current_sand.unwrap();
        if !self[cell].is_blocked() {
            self[current_sand] = Cell::Air;
            self[cell] = Cell::Sand;
            self.current_sand = Some(cell);
            true
        } else {
            false
        }
    }

    pub fn step_until_abyss(&mut self) {
        while !self.abyss_reached {
            self.step_abyss();
        }
    }

    // FLOOR VERSION

    pub fn step_floor(&mut self) {
        match self.current_sand {
            None => {
                if self[(500, 0)].is_blocked() {
                    self.source_reached = true;
                } else {
                self.current_sand = Some((500, 0));
                self[(500, 0)] = Cell::Sand;
                }
            }
            Some((s_x, s_y)) => {
                if !self.try_cell_floor((s_x, s_y + 1))
                    && !self.try_cell_floor((s_x - 1, s_y + 1))
                    && !self.try_cell_floor((s_x + 1, s_y + 1))
                {
                    self.current_sand = None;
                    self.number_of_rests += 1;
                }
            }
        }
    }

    fn try_cell_floor(&mut self, cell: (u32, u32)) -> bool {
        let current_sand = self.current_sand.unwrap();
        if cell.1 != self.floor && !self[cell].is_blocked() {
            self[current_sand] = Cell::Air;
            self[cell] = Cell::Sand;
            self.current_sand = Some(cell);
            true
        } else {
            false
        }
    }

    pub fn step_until_source_blocked(&mut self) {
        while !self.source_reached {
            self.step_floor();
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    impl Cave {
        fn dump_slice(
            &self,
            output: &mut impl std::fmt::Write,
            x_start: u32,
            x_end: u32,
            y_start: u32,
            y_end: u32,
        ) {
            for j in y_start..=y_end {
                for i in x_start..=x_end {
                    write!(output, "{}", self[(i, j)]).unwrap();
                }
                writeln!(output).unwrap();
            }
        }

        fn step_until_rest(&mut self) {
            self.step_abyss();
            while self.current_sand.is_some() {
                self.step_abyss();
            }
        }

        fn step_until_n_rests(&mut self, n: usize) {
            for _ in 0..n {
                self.step_until_rest();
            }
        }
    }


    #[test]
    fn cave_dump_slice() {
        // GIVEN
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let (_, cave) = Cave::parse(input).unwrap();

        // WHEN
        let mut output = String::new();
        cave.dump_slice(&mut output, 494, 503, 0, 10);

        // THEN
        assert_eq!(
            indoc! {"
            ......+...
            ..........
            ..........
            ..........
            ....#...##
            ....#...#.
            ..###...#.
            ........#.
            ........#.
            #########.
            ..........
            "},
            output
        );
    }

    #[test]
    fn cave_one_step() {
        // GIVEN
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let (_, mut cave) = Cave::parse(input).unwrap();

        // WHEN
        cave.step_abyss();

        // THEN
        let mut output = String::new();
        cave.dump_slice(&mut output, 494, 503, 0, 10);
        assert_eq!(
            indoc! {"
            ......+...
            ......o...
            ..........
            ..........
            ....#...##
            ....#...#.
            ..###...#.
            ........#.
            ........#.
            #########.
            ..........
            "},
            output
        );
    }

    #[test]
    fn cave_two_steps() {
        // GIVEN
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let (_, mut cave) = Cave::parse(input).unwrap();

        // WHEN
        cave.step_abyss();
        cave.step_abyss();

        // THEN
        let mut output = String::new();
        cave.dump_slice(&mut output, 494, 503, 0, 10);
        assert_eq!(
            indoc! {"
            ......+...
            ..........
            ......o...
            ..........
            ....#...##
            ....#...#.
            ..###...#.
            ........#.
            ........#.
            #########.
            ..........
            "},
            output
        );
    }

    #[test]
    fn cave_first_rest() {
        // GIVEN
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let (_, mut cave) = Cave::parse(input).unwrap();

        // WHEN
        cave.step_until_rest();

        // THEN
        let mut output = String::new();
        cave.dump_slice(&mut output, 494, 503, 0, 10);
        assert_eq!(
            indoc! {"
            ......+...
            ..........
            ..........
            ..........
            ....#...##
            ....#...#.
            ..###...#.
            ........#.
            ......o.#.
            #########.
            ..........
            "},
            output
        );
    }

    #[test]
    fn cave_five_rests() {
        // GIVEN
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let (_, mut cave) = Cave::parse(input).unwrap();

        // WHEN
        cave.step_until_n_rests(5);

        // THEN
        let mut output = String::new();
        cave.dump_slice(&mut output, 494, 503, 0, 10);
        assert_eq!(
            indoc! {"
            ......+...
            ..........
            ..........
            ..........
            ....#...##
            ....#...#.
            ..###...#.
            ......o.#.
            ....oooo#.
            #########.
            ..........
            "},
            output
        );
    }

    #[test]
    fn cave_rests_before_abyss() {
        // GIVEN
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let (_, mut cave) = Cave::parse(input).unwrap();

        // WHEN
        cave.step_until_abyss();

        // THEN
        assert_eq!(24, cave.number_of_rests());
    }

    #[test]
    fn cave_floor() {
    // GIVEN
    let input = indoc! {"
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    // WHEN
    let (_, cave) = Cave::parse(input).unwrap();

    // THEN
    assert_eq!(11, cave.floor);
    }

    #[test]
    fn cave_rests_before_source_blocked() {
        // GIVEN
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};
        let (_, mut cave) = Cave::parse(input).unwrap();

        // WHEN
        cave.step_until_source_blocked();

        // THEN
        assert_eq!(93, cave.number_of_rests());
    }
}
