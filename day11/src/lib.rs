use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

pub(crate) struct Monkey {
    items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    divisible_by: u32,
    dest_if_true: u32,
    dest_if_false: u32,
    inspect_count: u32,
}

use std::{collections::BinaryHeap, fmt};
impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

impl Monkey {
    pub fn from_string(input: &str) -> IResult<&str, Monkey> {
        let (input, (_, items, operation, (divisible_by, dest_if_true, dest_if_false))) =
            tuple((
                Self::monkey_header,
                Self::monkey_starting_items,
                Self::monkey_operation,
                Self::monkey_division_test,
            ))(input)?;
        Ok((
            input,
            Monkey {
                items,
                operation: Box::new(operation),
                divisible_by,
                dest_if_true,
                dest_if_false,
                inspect_count: 0,
            },
        ))
    }

    fn number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, |num: &str| num.parse())(input)
    }

    fn monkey_header(input: &str) -> IResult<&str, ()> {
        let (input, _) = terminated(tuple((tag("Monkey "), (digit1), tag(":"))), tag("\n"))(input)?;
        Ok((input, ()))
    }

    fn monkey_starting_items(input: &str) -> IResult<&str, Vec<u32>> {
        let (input, _) = tag("  Starting items: ")(input)?;
        terminated(separated_list1(tag(", "), Self::number), tag("\n"))(input)
    }

    fn monkey_operation(input: &str) -> IResult<&str, impl Fn(u32) -> u32> {
        let (input, _) = tag("  Operation: new = old ")(input)?;
        let (input, operator) = one_of("+*")(input)?;
        let operation = match operator {
            '+' => |x, y| x + y,
            '*' => |x, y| x * y,
            _ => unreachable!(),
        };
        let (input, _) = tag(" ")(input)?;

        let (input, operand) = terminated(alt((tag("old"), digit1)), tag("\n"))(input)?;
        let operand_copy = operand.to_string();
        Ok((input, move |x| {
            operation(x, operand_copy.parse().unwrap_or(x))
        }))
    }

    fn monkey_division_test(input: &str) -> IResult<&str, (u32, u32, u32)> {
        let (input, _) = tag("  Test: divisible by ")(input)?;
        let (input, divisible_by) = terminated(Self::number, tag("\n"))(input)?;
        let (input, _) = tag("    If true: throw to monkey ")(input)?;
        let (input, dest_if_true) = terminated(Self::number, tag("\n"))(input)?;
        let (input, _) = tag("    If false: throw to monkey ")(input)?;
        let (input, dest_if_false) = terminated(Self::number, tag("\n"))(input)?;

        Ok((input, (divisible_by, dest_if_true, dest_if_false)))
    }
}

#[derive(Debug)]
pub struct Puzzle {
    monkeys: Vec<Monkey>,
}

impl Puzzle {
    pub fn from_string(input: &str) -> IResult<&str, Self> {
        let (input, monkeys) = separated_list1(tag("\n"), Monkey::from_string)(input)?;
        Ok((input, Puzzle { monkeys }))
    }

    pub fn monkey_business(&mut self) -> u32 {
        for _ in 0..20 {
            self.round()
        }

        let mut heap = self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspect_count)
            .collect::<BinaryHeap<u32>>();

        let iter = std::iter::from_fn(|| heap.pop());
        iter.take(2).product()
    }

    fn round(&mut self) {
        for monkey_index in 0..self.monkeys.len() {
            for i in 0..self.monkeys[monkey_index].items.len() {
                self.monkeys[monkey_index].inspect_count += 1;
                let worry_level =
                    (self.monkeys[monkey_index].operation)(self.monkeys[monkey_index].items[i]) / 3;
                let dest = if worry_level % self.monkeys[monkey_index].divisible_by == 0 {
                    self.monkeys[monkey_index].dest_if_true
                } else {
                    self.monkeys[monkey_index].dest_if_false
                };
                self.monkeys[dest as usize].items.push(worry_level);
            }
            self.monkeys[monkey_index].items.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_header_test() {
        // GIVEN
        let input = "Monkey 2:\n";
        // WHEN
        let res = Monkey::monkey_header(input);
        // THEN
        assert_eq!(Ok(("", ())), res);
    }

    #[test]
    fn parse_starting_items() {
        // GIVEN
        let input = "  Starting items: 62, 92, 50\n";
        // WHEN
        let res = Monkey::monkey_starting_items(input);
        // THEN
        assert_eq!(Ok(("", vec![62, 92, 50])), res);
    }

    #[test]
    fn parse_operation_immediate() {
        // GIVEN
        let input = "  Operation: new = old + 5\n";
        // WHEN
        let (remainder, op) = Monkey::monkey_operation(input).unwrap();
        // THEN
        assert_eq!("", remainder);
        assert_eq!(12, op(7));
    }

    #[test]
    fn parse_operation_square() {
        // GIVEN
        let input = "  Operation: new = old * old\n";
        // WHEN
        let (remainder, op) = Monkey::monkey_operation(input).unwrap();
        // THEN
        assert_eq!("", remainder);
        assert_eq!(49, op(7));
    }

    #[test]
    fn parse_monkey_test() {
        // GIVEN
        let input = indoc! {"
            Monkey 7:
              Starting items: 84, 93, 54
              Operation: new = old + 1
              Test: divisible by 17
                If true: throw to monkey 2
                If false: throw to monkey 1
            "};
        // WHEN
        let (remainder, _monkey) = Monkey::from_string(input).unwrap();
        // THEN
        assert_eq!("", remainder);
    }

    #[test]
    fn monkey_business() {
        // GIVEN
        let input = indoc! {"
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
            "};

        let (remainder, mut puzzle) = Puzzle::from_string(input).unwrap();
        // WHEN
        let result = puzzle.monkey_business();

        // THEN
        assert_eq!("", remainder);
        assert_eq!(10605, result);
    }
}
