#![feature(iter_array_chunks)]
use std::collections::HashSet;

struct Rucksack {
    left: HashSet<u8>,
    right: HashSet<u8>,
}

impl Rucksack {
    fn from_line(line: &str) -> Self {
        let chars = line.as_bytes();
        let len = chars.len();
        let (left, right) = chars.split_at(len / 2);
        Rucksack {
            left: left.iter().copied().collect(),
            right: right.iter().copied().collect(),
        }
    }

    fn common_item(&self) -> Result<u8, &'static str> {
        self.left
            .intersection(&self.right)
            .next()
            .copied()
            .ok_or("No common item")
    }

    fn priority(item: u8) -> Result<u32, &'static str> {
        match item {
            b'a'..=b'z' => Ok((item - b'a' + 1).into()),
            b'A'..=b'Z' => Ok((item - b'A' + 27).into()),
            _ => Err("item is not a letter"),
        }
    }
}

struct Group([Rucksack; 3]);

impl Group {
    fn find_badge(&self) -> Result<u8, &'static str> {
        self.0
            .iter()
            .map(|rucksack| rucksack.left.union(&rucksack.right).copied().collect())
            .reduce(|acc: HashSet<u8>, rucksack| acc.intersection(&rucksack).copied().collect())
            .ok_or("Cannot find badge: empty group")
            .and_then(|set| set.iter().next().copied().ok_or("No badge found"))
    }
}

pub struct Inventory(Vec<Group>);

impl Inventory {
    pub fn from_string(input: &str) -> Self {
        Inventory(
            input
                .lines()
                .map(Rucksack::from_line)
                .array_chunks()
                .map(Group)
                .collect(),
        )
    }

    pub fn priority(&self) -> Result<u32, &'static str> {
        self.0
            .iter()
            .map(|group| {
                group
                    .0
                    .iter()
                    .map(|rucksack| rucksack.common_item().and_then(Rucksack::priority))
                    .sum::<Result<u32, &'static str>>()
            })
            .sum()
    }

    pub fn badge_priority(&self) -> Result<u32, &'static str> {
        self.0
            .iter()
            .map(|group| group.find_badge().and_then(Rucksack::priority))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksack_from_line() {
        // GIVEN
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";

        // WHEN
        let rucksack = Rucksack::from_line(line);

        // THEN
        assert_eq!(
            b"vJrwpWtwJgWr".iter().copied().collect::<HashSet<u8>>(),
            rucksack.left
        );
        assert_eq!(
            b"hcsFMMfFFhFp".iter().copied().collect::<HashSet<u8>>(),
            rucksack.right
        );
    }

    #[test]
    fn rucksack_common_item() {
        // GIVEN
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from_line(line);

        // WHEN
        let common_item = rucksack.common_item();

        // THEN
        assert_eq!(Ok(b'p'), common_item);
    }

    #[test]
    fn rucksack_priority_lowercase() {
        // GIVEN

        // WHEN
        let priority = Rucksack::priority(b'f');

        // THEN
        assert_eq!(Ok(6), priority);
    }

    #[test]
    fn rucksack_priority_uppercase() {
        // GIVEN
        let item = b'F';

        // WHEN
        let priority = Rucksack::priority(item);

        // THEN
        assert_eq!(Ok(32), priority);
    }

    #[test]
    fn group_find_badge() {
        // GIVEN
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
        let inventory = Inventory::from_string(input);
        let group = &inventory.0[0];

        // WHEN
        let badge = group.find_badge();

        // THEN
        assert_eq!(Ok(b'r'), badge);
    }

    #[test]
    fn inventory_priority() {
        // GIVEN
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let inventory = Inventory::from_string(input);

        // WHEN
        let priority = inventory.priority();

        // THEN
        assert_eq!(Ok(157), priority);
    }

    #[test]
    fn inventory_badge_priority() {
        // GIVEN
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let inventory = Inventory::from_string(input);

        // WHEN
        let badge_priority = inventory.badge_priority();

        // THEN
        assert_eq!(Ok(70), badge_priority);
    }
}
