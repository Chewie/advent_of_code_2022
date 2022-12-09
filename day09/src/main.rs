use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut rope2 = day09::Rope::new(2);
    rope2.apply_from_string(&input).unwrap();

    let unique_positions2 = rope2.unique_tail_positions();

    println!("Unique tail positions on rope 2: {unique_positions2}");

    let mut rope10 = day09::Rope::new(10);
    rope10.apply_from_string(&input).unwrap();

    let unique_positions10 = rope10.unique_tail_positions();

    println!("Unique tail positions on rope 10: {unique_positions10}");
}
