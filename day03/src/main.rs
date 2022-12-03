use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let inventory = day03::Inventory::from_string(input);
    let priority = inventory.priority();
    println!("Inventory priority: {priority}");
}
