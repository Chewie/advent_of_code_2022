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

enum Dir {
    L,
    U,
    R,
    D,
}

struct Command {
    direction: Dir,
    amount: usize,
}

impl Command {
    fn new(line: &str) -> Result<Self> {
        let (dir, amount) = line.split_once(' ').ok_or("Cannot split")?;
        let dir = match dir {
            "L" => Dir::L,
            "U" => Dir::U,
            "R" => Dir::R,
            "D" => Dir::D,
            _ => return Err(Box::from("unknown command")),
        };
        Ok(Command {
            direction: dir,
            amount: amount.parse()?,
        })
    }
}

pub struct Rope {
    head: Point,
    tail: Point,
    visited_by_tail: HashSet<Point>,
}

impl Default for Rope {
    fn default() -> Self {
        Self::new()
    }
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
            visited_by_tail: HashSet::from([Point { x: 0, y: 0 }]),
        }
    }

    pub fn apply_from_string(&mut self, input: &str) -> Result<()> {
        let commands: Vec<Command> = input.lines().map(Command::new).collect::<Result<_>>()?;

        for command in commands {
            for _ in 0..command.amount {
                let dir_vec = match command.direction {
                    Dir::L => Point { x: -1, y: 0 },
                    Dir::U => Point { x: 0, y: 1 },
                    Dir::R => Point { x: 1, y: 0 },
                    Dir::D => Point { x: 0, y: -1 },
                };
                self.head += dir_vec;
                self.update_tail();
            }
        }
        Ok(())
    }

    pub fn unique_tail_positions(&self) -> usize {
        self.visited_by_tail.len()
    }

    fn update_tail(&mut self) {
        let delta = self.head - self.tail;

        if std::cmp::max(delta.x.abs(), delta.y.abs()) > 1 {
            let tail_dir_vec = match (delta.x, delta.y) {
                (2, 0) => Point { x: 1, y: 0 },
                (-2, 0) => Point { x: -1, y: 0 },
                (0, 2) => Point { x: 0, y: 1 },
                (0, -2) => Point { x: 0, y: -1 },
                (2, 1) | (1, 2) => Point { x: 1, y: 1 },
                (2, -1) | (1, -2) => Point { x: 1, y: -1 },
                (-2, 1) | (-1, 2) => Point { x: -1, y: 1 },
                (-2, -1) | (-1, -2) => Point { x: -1, y: -1 },
                _ => unreachable!(),
            };
            self.tail += tail_dir_vec;
            self.visited_by_tail.insert(self.tail);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn unique_tail_positions() {
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
        let mut rope = Rope::new();

        // WHEN
        rope.apply_from_string(input).unwrap();

        // THEN
        assert_eq!(13, rope.unique_tail_positions());
    }
}
