use std::io::Read;

mod part1;
mod part2;

fn main() {
    let mut lines = String::new();
    std::io::stdin().read_to_string(&mut lines).unwrap();

    let p1_strategy = part1::Strategy::from_string(lines.clone());
    let p1_score = p1_strategy.predict_score();

    println!("Part 1 predicted score: {p1_score}");

    let p2_strategy = part2::Strategy::from_string(lines);
    let p2_score = p2_strategy.predict_score();

    println!("Part 2 predicted score: {p2_score}");
}
