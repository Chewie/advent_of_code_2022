use std::io::Read;

fn main() {
    let mut lines = String::new();
    std::io::stdin().read_to_string(&mut lines).unwrap();

    let strategy = day02::Strategy::from_string(lines);
    let score = strategy.predict_score();

    println!("Predicted score: {score}");
}
