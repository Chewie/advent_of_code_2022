type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

struct Command {
    direction: Point,
    amount: usize,
}

impl Command {
    fn new(line: &str) -> Result<Self> {
        let (dir, amount) = line.split_once(' ').ok_or("Cannot split")?;
        let dir = match dir {
            "L" => Point { x: -1, y: 0 },
            "U" => Point { x: 0, y: 1 },
            "R" => Point { x: 1, y: 0 },
            "D" => Point { x: 0, y: -1 },
            _ => return Err(Box::from("unknown command")),
        };
        Ok(Command {
            direction: dir,
            amount: amount.parse()?,
        })
    }
}

pub struct Rope {
    knots: Vec<Point>,
    visited_by_tail: HashSet<Point>,
}

impl Rope {
    pub fn new(size: usize) -> Self {
        Rope {
            knots: vec![Point { x: 0, y: 0 }; size],
            visited_by_tail: HashSet::from([Point { x: 0, y: 0 }]),
        }
    }

    pub fn apply_from_string(&mut self, input: &str) -> Result<()> {
        let commands: Vec<Command> = input.lines().map(Command::new).collect::<Result<_>>()?;

        for command in commands {
            for _ in 0..command.amount {
                self.knots[0] += command.direction;
                self.update_knots();
            }
        }
        Ok(())
    }

    pub fn unique_tail_positions(&self) -> usize {
        self.visited_by_tail.len()
    }

    fn update_knots(&mut self) {
        for i in 1..self.knots.len() {
            let delta = self.knots[i - 1] - self.knots[i];
            if delta.x.abs() > 1 || delta.y.abs() > 1 {
                let dir_vec = match (delta.x, delta.y) {
                    (2, 0) => Point { x: 1, y: 0 },
                    (-2, 0) => Point { x: -1, y: 0 },
                    (0, 2) => Point { x: 0, y: 1 },
                    (0, -2) => Point { x: 0, y: -1 },
                    (2, 1) | (1, 2) | (2, 2) => Point { x: 1, y: 1 },
                    (2, -1) | (1, -2) | (2, -2) => Point { x: 1, y: -1 },
                    (-2, 1) | (-1, 2) | (-2, 2) => Point { x: -1, y: 1 },
                    (-2, -1) | (-1, -2) | (-2, -2) => Point { x: -1, y: -1 },
                    (x, y) => unreachable!("({x}, {y})"),
                };
                self.knots[i] += dir_vec;
            }
        }
        self.visited_by_tail.insert(*self.knots.last().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn rope2_unique_tail_positions() {
        // GIVEN
        let input = indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "};
        let mut rope = Rope::new(2);

        // WHEN
        rope.apply_from_string(input).unwrap();

        // THEN
        assert_eq!(13, rope.unique_tail_positions());
    }

    #[test]
    fn rope10_unique_tail_positions() {
        // GIVEN
        let input = indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "};
        let mut rope = Rope::new(10);

        // WHEN
        rope.apply_from_string(input).unwrap();

        // THEN
        assert_eq!(1, rope.unique_tail_positions());
    }

    #[test]
    fn rope10_unique_tail_positions_large() {
        // GIVEN
        let input = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "};
        let mut rope = Rope::new(10);

        // WHEN
        rope.apply_from_string(input).unwrap();

        // THEN
        assert_eq!(36, rope.unique_tail_positions());
    }
}
