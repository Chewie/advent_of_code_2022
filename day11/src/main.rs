use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (_, mut puzzle1) = day11::Puzzle::from_string(&input).unwrap();
    let monkey_business = puzzle1.monkey_business(20, false);
    println!("Monkey business: {monkey_business}");

    let (_, mut puzzle2) = day11::Puzzle::from_string(&input).unwrap();
    let ridiculous_monkey_business = puzzle2.monkey_business(10000, true);
    println!("Ridiculous monkey business: {ridiculous_monkey_business}");
}
