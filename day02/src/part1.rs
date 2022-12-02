use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
use Move::*;

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Cannot parse Move: invalid char"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent_move: Move,
    your_move: Move,
}

impl Round {
    fn from_line(line: &str) -> Result<Self, &'static str> {
        let (opponent, your) = line
            .split_once(" ")
            .ok_or("Cannot create Round: unable to split")?;
        match (opponent.parse(), your.parse()) {
            (Ok(opponent_move), Ok(your_move)) => Ok(Round {
                opponent_move,
                your_move,
            }),
            _ => Err("Cannot create Round: Move parse failed"),
        }
    }

    fn score(&self) -> u32 {
        self.shape_score() + self.outcome_score()
    }

    fn shape_score(&self) -> u32 {
        self.your_move as u32
    }

    fn outcome_score(&self) -> u32 {
        match (self.your_move, self.opponent_move) {
            (Rock, Paper) => 0,
            (Rock, Rock) => 3,
            (Rock, Scissors) => 6,
            (Paper, Scissors) => 0,
            (Paper, Paper) => 3,
            (Paper, Rock) => 6,
            (Scissors, Rock) => 0,
            (Scissors, Scissors) => 3,
            (Scissors, Paper) => 6,
        }
    }
}

pub struct Strategy(Vec<Round>);

impl Strategy {
    pub fn from_string(input: impl AsRef<str>) -> Strategy {
        Strategy(
            input
                .as_ref()
                .lines()
                .filter_map(|line| Round::from_line(line).ok())
                .collect(),
        )
    }

    pub fn predict_score(&self) -> u32 {
        self.0.iter().map(|round| round.score()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_from_string_empty() {
        // GIVEN
        let input = "";

        // WHEN
        let strategy = Strategy::from_string(input);

        // THEN
        let expected_rounds: Vec<Round> = vec![];
        assert_eq!(expected_rounds, strategy.0);
    }

    #[test]
    fn strategy_from_string_multiline() {
        // GIVEN
        let input = "A Y\nB X\nC Z\n";

        // WHEN
        let strategy = Strategy::from_string(input);

        // THEN
        let expected_rounds = vec![
            Round {
                opponent_move: Rock,
                your_move: Paper,
            },
            Round {
                opponent_move: Paper,
                your_move: Rock,
            },
            Round {
                opponent_move: Scissors,
                your_move: Scissors,
            },
        ];
        assert_eq!(expected_rounds, strategy.0);
    }

    #[test]
    fn predict_score_rock_vs_paper() {
        // GIVEN
        let input = "A Y\n";
        let strategy = Strategy::from_string(input);

        // WHEN
        let score = strategy.predict_score();

        // THEN
        assert_eq!(8, score);
    }

    #[test]
    fn predict_score_paper_vs_rock() {
        // GIVEN
        let input = "B X\n";
        let strategy = Strategy::from_string(input);

        // WHEN
        let score = strategy.predict_score();

        // THEN
        assert_eq!(1, score);
    }

    #[test]
    fn predict_score_three_rounds() {
        // GIVEN
        let input = "A Y\nB X\nC Z\n";
        let strategy = Strategy::from_string(input);

        // WHEN
        let score = strategy.predict_score();

        // THEN
        assert_eq!(15, score);
    }
}
