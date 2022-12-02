type Elf = Vec<u32>;

#[derive(Debug, PartialEq)]
pub struct Inventory(Vec<Elf>);

impl Inventory {
    pub fn from_string<T: AsRef<str>>(string: T) -> Inventory {
        Inventory(
            string
                .as_ref()
                .trim()
                .split("\n\n")
                .map(|single_elf| {
                    single_elf
                        .split("\n")
                        .map(|item| item.parse().unwrap())
                        .collect()
                })
                .collect(),
        )
    }

    pub fn highest_sum(&self) -> u32 {
        let calories = self.0.iter().map(|v| v.iter().sum());
        calories.max().unwrap()
    }

    pub fn highest_three_sum(&self) -> u32 {
        use std::collections::BinaryHeap;

        let mut heap = self
            .0
            .iter()
            .map(|elf| elf.iter().sum())
            .collect::<BinaryHeap<u32>>();

        // Poor man's into_iter_sorted
        let iter = std::iter::from_fn(|| heap.pop());
        iter.take(3).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_inventory_one_item() {
        // GIVEN
        let string = "42\n";

        let expected_inventory = Inventory(vec![vec![42]]);

        // WHEN
        let result = Inventory::from_string(string);

        // THEN
        assert_eq!(expected_inventory, result);
    }

    #[test]
    fn construct_inventory_one_elf_two_items() {
        // GIVEN
        let string = "42\n51\n";

        let expected_inventory = Inventory(vec![vec![42, 51]]);

        // WHEN
        let result = Inventory::from_string(string);

        // THEN
        assert_eq!(expected_inventory, result);
    }

    #[test]
    fn construct_inventory_many_elf_many_items() {
        // GIVEN
        let string = "1\n2\n\n3\n\n4\n5\n";

        let expected_inventory = Inventory(vec![vec![1, 2], vec![3], vec![4, 5]]);

        // WHEN
        let result = Inventory::from_string(string);

        // THEN
        assert_eq!(expected_inventory, result);
    }

    #[test]
    fn highest_sum_one_item() {
        // GIVEN
        let inventory = Inventory(vec![vec![42]]);

        // WHEN
        let result = inventory.highest_sum();

        // THEN
        assert_eq!(42, result);
    }

    #[test]
    fn highest_sum_two_elves_with_one_item_each() {
        // GIVEN
        let inventory = Inventory(vec![vec![42], vec![51]]);

        // WHEN
        let result = inventory.highest_sum();

        // THEN
        assert_eq!(51, result);
    }

    #[test]
    fn highest_sum_many_elves_many_items() {
        // GIVEN
        let inventory = Inventory(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        // WHEN
        let result = inventory.highest_sum();

        // THEN
        assert_eq!(24, result);
    }

    #[test]
    fn highest_three_sum_test() {
        // GIVEN
        let inventory = Inventory(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ]);

        // WHEN
        let result = inventory.highest_three_sum();

        // THEN
        assert_eq!(72, result);
    }
}
