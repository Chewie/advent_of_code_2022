use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let forest = day08::Forest::from_string(&input);
    let total_visible = forest.number_of_visibles();
    let highest_score = forest.highest_scenic_score();

    println!("Number of visibles: {total_visible}");
    println!("Highest scenic score: {highest_score}");
}
