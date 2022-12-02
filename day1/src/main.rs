use std::io::Read;

fn main() {
    let mut lines = String::new();
    std::io::stdin().read_to_string(&mut lines).unwrap();

    let inventory = day1::construct_inventory(lines);
    let sum = day1::highest_sum(inventory);
    println!("Highest sum: {sum}");
}
