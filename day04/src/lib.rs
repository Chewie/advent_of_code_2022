type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Assignment {
    low: usize,
    high: usize,
}

impl Assignment {
    fn fully_contains(&self, other: &Self) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn from_string(input: impl AsRef<str>) -> Result<Self> {
        let (low, high) = input
            .as_ref()
            .split_once("-")
            .ok_or("Cannot construct assignment")?;
        Ok(Assignment {
            low: low.parse()?,
            high: high.parse()?,
        })
    }
}

struct Pair {
    left: Assignment,
    right: Assignment,
}

impl Pair {
    fn either_contains_the_other(&self) -> bool {
        self.left.fully_contains(&self.right) || self.right.fully_contains(&self.left)
    }

    fn from_string(input: impl AsRef<str>) -> Result<Self> {
        let (left, right) = input
            .as_ref()
            .split_once(",")
            .ok_or("Cannot construct pair")?;
        Ok(Pair {
            left: Assignment::from_string(left)?,
            right: Assignment::from_string(right)?,
        })
    }
}

pub struct WorkSheet(Vec<Pair>);

impl WorkSheet {
    pub fn from_string(input: impl AsRef<str>) -> Result<Self> {
        Ok(WorkSheet(
            input
                .as_ref()
                .lines()
                .map(Pair::from_string)
                .collect::<Result<Vec<_>>>()?,
        ))
    }

    pub fn count_fully_contains(&self) -> usize {
        self.0
            .iter()
            .filter(|pair| pair.either_contains_the_other())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assignment_fully_contains_true() {
        // GIVEN
        let assignment1 = Assignment { low: 2, high: 8 };
        let assignment2 = Assignment { low: 3, high: 7 };

        // WHEN
        let result = assignment1.fully_contains(&assignment2);

        // THEN
        assert_eq!(true, result);
    }

    #[test]
    fn assignment_fully_contains_false() {
        // GIVEN
        let assignment1 = Assignment { low: 5, high: 7 };
        let assignment2 = Assignment { low: 7, high: 9 };

        // WHEN
        let result = assignment1.fully_contains(&assignment2);

        // THEN
        assert_eq!(false, result);
    }

    #[test]
    fn pair_either_contains_the_other() {
        // GIVEN
        let pair = Pair {
            left: Assignment { low: 2, high: 8 },
            right: Assignment { low: 3, high: 7 },
        };

        // WHEN
        let result = pair.either_contains_the_other();

        // THEN
        assert_eq!(true, result);
    }

    #[test]
    fn worksheet_count_fully_contains() {
        // GIVEN
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let worksheet = WorkSheet::from_string(input).unwrap();

        // WHEN
        let result = worksheet.count_fully_contains();

        // THEN
        assert_eq!(2, result);
    }
}
