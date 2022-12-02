use std::io::Read;

fn main() {
    let mut lines = String::new();
    std::io::stdin().read_to_string(&mut lines).unwrap();

    let inventory = day1::Inventory::from_string(lines);
    let sum = inventory.highest_sum();
    let three_sum = inventory.highest_three_sum();

    println!("Highest sum: {sum}");
    println!("Highest three sum: {three_sum}");
}
