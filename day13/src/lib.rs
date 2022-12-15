use itertools::EitherOrBoth::{Both, Left, Right};
use std::cmp::Ordering;

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Num(usize),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}


impl Packet {
    fn parse(input: &str) -> IResult<&str, Packet> {
        alt((
            map(digit1, |d: &str| Packet::Num(d.parse().unwrap())),
            delimited(
                char('['),
                map(separated_list0(char(','), Packet::parse), Packet::List),
                char(']'),
            ),
        ))(input)
    }

    fn compare(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => self.handle_two_lists(l, r),
            (l @ Packet::Num(_), Packet::List(r)) => {
                self.handle_two_lists(std::slice::from_ref(l), r)
            }
            (Packet::List(l), r @ Packet::Num(_)) => {
                self.handle_two_lists(l, std::slice::from_ref(r))
            }
        }
    }

    fn handle_two_lists(&self, l: &[Packet], r: &[Packet]) -> Ordering {
        for it in l.iter().zip_longest(r.iter()) {
            match it {
                Right(_) => return Ordering::Less,
                Left(_) => return Ordering::Greater,
                Both(l, r) => match l.compare(r) {
                    Ordering::Equal => continue,
                    order => return order,
                },
            }
        }
        Ordering::Equal
    }
}

#[derive(Debug, PartialEq)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    pub fn parse(input: &str) -> IResult<&str, Pair> {
        map(
            tuple((
                terminated(Packet::parse, char('\n')),
                terminated(Packet::parse, char('\n')),
            )),
            |(left, right)| Pair { left, right },
        )(input)
    }

    pub fn compare(&self) -> Ordering {
        self.left.compare(&self.right)
    }
}

pub struct Signal {
    pairs: Vec<Pair>,
}

impl Signal {
    pub fn parse(input: &str) -> IResult<&str, Signal> {
        map(separated_list0(char('\n'), Pair::parse), |pairs| Signal {
            pairs,
        })(input)
    }

    pub fn sum_indices_in_right_order(&self) -> usize {
        (1..)
            .zip(self.pairs.iter())
            .filter(|(_, pair)| pair.compare() == Ordering::Less)
            .map(|(i, _)| i)
            .sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use Packet::{List, Num};

    #[test]
    fn parse_packet_simple_list() {
        // GIVEN
        let input = "[1,1,3,1,1]";

        // WHEN
        let (remainder, packet) = Packet::parse(input).unwrap();

        // THEN
        assert_eq!("", remainder);
        assert_eq!(List(vec![Num(1), Num(1), Num(3), Num(1), Num(1),]), packet);
    }

    #[test]
    fn parse_packet_nested_list() {
        // GIVEN
        let input = "[[1],[2,3,4]]";

        // WHEN
        let (remainder, packet) = Packet::parse(input).unwrap();

        // THEN
        assert_eq!("", remainder);
        assert_eq!(
            List(vec![List(vec![Num(1)]), List(vec![Num(2), Num(3), Num(4)])]),
            packet
        );
    }

    #[test]
    fn parse_packet_empty_list() {
        // GIVEN
        let input = "[[[]]]";

        // WHEN
        let (remainder, packet) = Packet::parse(input).unwrap();

        // THEN
        assert_eq!("", remainder);
        assert_eq!(List(vec![List(vec![List(vec![])])]), packet);
    }

    #[test]
    fn parse_pair_test() {
        // GIVEN
        let input = indoc! {"
            [1,1,3,1,1]
            [1,1,5,1,1]
        "};

        // WHEN
        let (remainder, pair) = Pair::parse(input).unwrap();

        // THEN
        assert_eq!("", remainder);
        assert_eq!(
            Pair {
                left: List(vec![Num(1), Num(1), Num(3), Num(1), Num(1),]),
                right: List(vec![Num(1), Num(1), Num(5), Num(1), Num(1),])
            },
            pair
        );
    }

    #[test]
    fn right_order_simple_list() {
        // GIVEN
        let input = indoc! {"
            [1,1,3,1,1]
            [1,1,5,1,1]
        "};
        let (_, pair) = Pair::parse(input).unwrap();

        // WHEN
        let result = pair.compare();

        // THEN
        assert_eq!(Ordering::Less, result);
    }

    #[test]
    fn right_order_nested_list() {
        // GIVEN
        let input = indoc! {"
            [[4,4],4,4]
            [[4,4],4,4,4]
            "};
        let (_, pair) = Pair::parse(input).unwrap();

        // WHEN
        let result = pair.compare();

        // THEN
        assert_eq!(Ordering::Less, result);
    }

    #[test]
    fn right_order_mixed_types() {
        // GIVEN
        let input = indoc! {"
            [9]
            [[8,7,6]]
            "};
        let (_, pair) = Pair::parse(input).unwrap();

        // WHEN
        let result = pair.compare();

        // THEN
        assert_eq!(Ordering::Greater, result);
    }

    #[test]
    fn signal_sum_indices_in_right_order() {
        // GIVEN
        let input = indoc! {"
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
            "};
        let (_, signal) = Signal::parse(input).unwrap();

        // WHEN
        let result = signal.sum_indices_in_right_order();

        // THEN
        assert_eq!(13, result);
    }
}
