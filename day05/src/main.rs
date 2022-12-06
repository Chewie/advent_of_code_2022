use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut puzzle = day05::Puzzle::from_string(&input);
    puzzle.apply_commands();
    let top_row = puzzle.top_row();

    println!("Top row: {top_row}");
}
