type Elf = Vec<u32>;
type Inventory = Vec<Elf>;

pub fn highest_sum(total: Inventory) -> u32 {
    let calories = total.iter().map(|v| v.iter().sum());
    calories.max().unwrap()
}

pub fn construct_inventory<T: AsRef<str>>(lines: T) -> Inventory {
    lines
        .as_ref()
        .trim()
        .split("\n\n")
        .map(|single_elf| {
            single_elf
                .split("\n")
                .map(|item| item.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highest_sum_one_item() {
        // GIVEN
        let total_inventory = vec![vec![42]];

        // WHEN
        let result = highest_sum(total_inventory);

        // THEN
        assert_eq!(42, result);
    }

    #[test]
    fn highest_sum_two_elves_with_one_item_each() {
        // GIVEN
        let total_inventory = vec![vec![42], vec![51]];

        // WHEN
        let result = highest_sum(total_inventory);

        // THEN
        assert_eq!(51, result);
    }

    #[test]
    fn highest_sum_many_elves_many_items() {
        // GIVEN
        let total_inventory = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        // WHEN
        let result = highest_sum(total_inventory);

        // THEN
        assert_eq!(24, result);
    }

    #[test]
    fn construct_inventory_one_item() {
        // GIVEN
        let lines = "42\n";

        let expected_inventory = vec![vec![42]];

        // WHEN
        let result = construct_inventory(lines);

        // THEN
        assert_eq!(expected_inventory, result);
    }

    #[test]
    fn construct_inventory_one_elf_two_items() {
        // GIVEN
        let lines = "42\n51\n";

        let expected_inventory = vec![vec![42, 51]];

        // WHEN
        let result = construct_inventory(lines);

        // THEN
        assert_eq!(expected_inventory, result);
    }

    #[test]
    fn construct_inventory_many_elf_many_items() {
        // GIVEN
        let lines = "1\n2\n\n3\n\n4\n5\n";

        let expected_inventory = vec![vec![1, 2], vec![3], vec![4, 5]];

        // WHEN
        let result = construct_inventory(lines);

        // THEN
        assert_eq!(expected_inventory, result);
    }
}
