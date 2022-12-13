use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (_, mut puzzle) = day11::Puzzle::from_string(&input).unwrap();

    let monkey_business = puzzle.monkey_business();

    println!("Monkey business: {monkey_business}");
}
