use std::io::Read;

fn main() -> Result<(), &'static str> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let inventory = day03::Inventory::from_string(&input);
    let priority = inventory.priority()?;
    println!("Inventory priority: {priority}");

    let badge_priority = inventory.badge_priority()?;
    println!("Inventory badge priority: {badge_priority}");

    Ok(())
}
