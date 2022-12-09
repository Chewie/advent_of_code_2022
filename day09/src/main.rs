use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut rope = day09::Rope::new();
    rope.apply_from_string(&input).unwrap();

    let unique_positions = rope.unique_tail_positions();

    println!("Unique tail positions: {unique_positions}");
}
