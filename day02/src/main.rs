use std::io::Read;

mod part1;

fn main() {
    let mut lines = String::new();
    std::io::stdin().read_to_string(&mut lines).unwrap();

    let strategy = part1::Strategy::from_string(lines);
    let score = strategy.predict_score();

    println!("Part 1 predicted score: {score}");
}
