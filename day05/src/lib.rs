use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default)]
struct Dock([Vec<u8>; 9]);

impl Dock {
    fn from_string(s: &str) -> Self {
        let mut dock = Dock::default();
        for row in s.lines().map(Dock::row_from_line) {
            for (stack, maybe_cargo) in dock.0.iter_mut().zip(row) {
                if let Some(cargo) = maybe_cargo {
                    stack.push(cargo);
                }
            }
        }
        for stack in dock.0.iter_mut() {
            stack.reverse();
        }
        dock
    }

    fn row_from_line(line: &str) -> Vec<Option<u8>> {
        line.as_bytes()
            .chunks(4)
            .map(|chunk| match chunk[1] {
                letter @ b'A'..=b'Z' => Some(letter),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Command {
    amount: usize,
    src: usize,
    dst: usize,
}

impl Command {
    fn from_string(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
        }

        let cap = RE.captures(input).unwrap();
        Command {
            amount: cap[1].parse().unwrap(),
            src: cap[2].parse().unwrap(),
            dst: cap[3].parse().unwrap(),
        }
    }
}

pub struct Puzzle {
    dock: Dock,
    commands: Vec<Command>,
}

impl Puzzle {
    pub fn from_string(input: &str) -> Self {
        let (dock_input, command_input) = input.split_once("\n\n").unwrap();
        Puzzle {
            dock: Dock::from_string(dock_input),
            commands: command_input.lines().map(Command::from_string).collect(),
        }
    }

    pub fn apply_commands(&mut self) {
        for command in &self.commands {
            for _ in 0..command.amount {
                let item = self.dock.0[command.src - 1].pop().unwrap();
                self.dock.0[command.dst - 1].push(item);
            }
        }
    }

    pub fn top_row(&self) -> String {
        String::from_utf8(
            self.dock
                .0
                .iter()
                .filter_map(|stack| stack.last())
                .copied()
                .collect(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_from_line() {
        // GIVEN
        let line = "        [F] [Q]         [Q]        ";

        // WHEN
        let row = Dock::row_from_line(line);

        // THEN
        assert_eq!(
            vec![
                None,
                None,
                Some(b'F'),
                Some(b'Q'),
                None,
                None,
                Some(b'Q'),
                None,
                None
            ],
            row
        );
    }

    #[test]
    fn dock_from_string() {
        // GIVEN
        let input = "        [F] [Q]         [Q]        
[B]     [Q] [V] [D]     [S]        
 1   2   3   4   5   6   7   8   9 ";

        // WHEN
        let dock = Dock::from_string(input);

        // THEN
        assert_eq!(
            vec![
                vec![b'B'],
                vec![],
                vec![b'Q', b'F'],
                vec![b'V', b'Q'],
                vec![b'D'],
                vec![],
                vec![b'S', b'Q'],
                vec![],
                vec![]
            ],
            dock.0
        );
    }

    #[test]
    fn command_from_string() {
        // GIVEN
        let input = "move 3 from 5 to 2";

        // WHEN
        let command = Command::from_string(input);

        // THEN
        assert_eq!(
            Command {
                amount: 3,
                src: 5,
                dst: 2
            },
            command
        );
    }

    #[test]
    fn puzzle_top_row() {
        // GIVEN
        let input = "
        [F] [Q]         [Q]        
[B]     [Q] [V] [D]     [S]        
[S] [P] [T] [R] [M]     [D]        
 1   2   3   4   5   6   7   8   9 

move 1 from 3 to 1
move 2 from 7 to 2";

        // WHEN
        let mut puzzle = Puzzle::from_string(input);
        puzzle.apply_commands();
        let top_row = puzzle.top_row();

        // THEN
        assert_eq!("FSQQDD", top_row);
    }
}
