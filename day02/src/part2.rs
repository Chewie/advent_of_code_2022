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
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            _ => Err("Cannot parse Move: invalid char"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}
use Outcome::*;

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err("Cannot parse Outcome: invalid char"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent_move: Move,
    desired_outcome: Outcome,
}

impl Round {
    fn from_line(line: &str) -> Result<Self, &'static str> {
        let (opponent, outcome) = line
            .split_once(' ')
            .ok_or("Cannot create Round: unable to split")?;
        match (opponent.parse(), outcome.parse()) {
            (Ok(opponent_move), Ok(desired_outcome)) => Ok(Round {
                opponent_move,
                desired_outcome,
            }),
            _ => Err("Cannot create Round: Move parse failed"),
        }
    }

    fn score(&self) -> u32 {
        self.shape_score() + self.outcome_score()
    }

    fn shape_score(&self) -> u32 {
        let your_move = match (self.opponent_move, self.desired_outcome) {
            (Rock, Win) => Paper,
            (Rock, Draw) => Rock,
            (Rock, Loss) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Draw) => Paper,
            (Paper, Loss) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Draw) => Scissors,
            (Scissors, Loss) => Paper,
        };
        your_move as u32
    }

    fn outcome_score(&self) -> u32 {
        self.desired_outcome as u32
    }
}

pub struct Strategy(Vec<Round>);

impl Strategy {
    pub fn from_string(input: &str) -> Strategy {
        Strategy(
            input
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
                desired_outcome: Draw,
            },
            Round {
                opponent_move: Paper,
                desired_outcome: Loss,
            },
            Round {
                opponent_move: Scissors,
                desired_outcome: Win,
            },
        ];
        assert_eq!(expected_rounds, strategy.0);
    }

    #[test]
    fn predict_score_draw_vs_paper() {
        // GIVEN
        let input = "A Y\n";
        let strategy = Strategy::from_string(input);

        // WHEN
        let score = strategy.predict_score();

        // THEN
        assert_eq!(4, score);
    }

    #[test]
    fn predict_score_lose_vs_rock() {
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
        assert_eq!(12, score);
    }
}
