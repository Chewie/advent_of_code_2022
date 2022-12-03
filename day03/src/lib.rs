use std::collections::HashSet;

struct Rucksack {
    left: HashSet<u8>,
    right: HashSet<u8>,
}

impl Rucksack {
    fn from_line(line: impl AsRef<str>) -> Self {
        let chars = line.as_ref().as_bytes();
        let len = chars.len();
        Rucksack {
            left: chars[..len / 2].iter().copied().collect(),
            right: chars[len / 2..].iter().copied().collect(),
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

pub struct Inventory(Vec<Rucksack>);

impl Inventory {
    pub fn from_string(input: impl AsRef<str>) -> Self {
        Inventory(input.as_ref().lines().map(Rucksack::from_line).collect())
    }

    pub fn priority(&self) -> u32 {
        self.0
            .iter()
            .filter_map(|rucksack| {
                rucksack
                    .common_item()
                    .and_then(|item| Rucksack::priority(item))
                    .ok()
            })
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
    fn inventory_priority() {
        // GIVEN
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        // WHEN
        let inventory = Inventory::from_string(input);

        // THEN
        assert_eq!(157, inventory.priority());
    }
}
