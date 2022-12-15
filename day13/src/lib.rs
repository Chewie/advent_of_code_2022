use std::{cmp::Ordering, collections::BTreeSet};

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Num(usize),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (l @ Packet::Num(_), Packet::List(r)) => std::slice::from_ref(l).cmp(r),
            (Packet::List(l), r @ Packet::Num(_)) => l.as_slice().cmp(std::slice::from_ref(r)),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn key1() -> Packet {
        Packet::List(vec![Packet::List(vec![Packet::Num(2)])])
    }

    fn key2() -> Packet {
        Packet::List(vec![Packet::List(vec![Packet::Num(6)])])
    }

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

}

#[derive(Debug, PartialEq)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    fn parse(input: &str) -> IResult<&str, Pair> {
        map(
            tuple((
                terminated(Packet::parse, char('\n')),
                terminated(Packet::parse, char('\n')),
            )),
            |(left, right)| Pair { left, right },
        )(input)
    }

    fn compare(&self) -> Ordering {
        self.left.cmp(&self.right)
    }
}

pub struct PairSignal {
    pairs: Vec<Pair>,
}

impl PairSignal {
    pub fn parse(input: &str) -> IResult<&str, PairSignal> {
        map(separated_list0(char('\n'), Pair::parse), |pairs| {
            PairSignal { pairs }
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

pub struct OrderedSignal {
    packets: BTreeSet<Packet>,
}

impl OrderedSignal {
    pub fn parse(input: &str) -> IResult<&str, OrderedSignal> {
        map(
            separated_list0(many1(char('\n')), Packet::parse),
            OrderedSignal::new,
        )(input)
    }

    fn new(packets: Vec<Packet>) -> OrderedSignal {
        let mut set: BTreeSet<Packet> = packets.into_iter().collect();
        set.insert(Packet::key1());
        set.insert(Packet::key2());
        OrderedSignal { packets: set }
    }

    pub fn decoder_key(&self) -> usize {
        let key1_pos = self
            .packets
            .iter()
            .position(|x| *x == Packet::key1())
            .unwrap()
            + 1;
        let key2_pos = self
            .packets
            .iter()
            .position(|x| *x == Packet::key2())
            .unwrap()
            + 1;
        key1_pos * key2_pos
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
    fn pair_signal_sum_indices_in_right_order() {
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
        let (_, signal) = PairSignal::parse(input).unwrap();

        // WHEN
        let result = signal.sum_indices_in_right_order();

        // THEN
        assert_eq!(13, result);
    }

    #[test]
    fn ordered_signal_decoder_key() {
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
        let (_, signal) = OrderedSignal::parse(input).unwrap();

        // WHEN
        let result = signal.decoder_key();

        // THEN
        assert_eq!(140, result);
    }
}
