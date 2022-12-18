use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let graph = day16::Graph::parse(&input);

    println!("Max pressure after 30mins: {}", graph.max_pressure());
}
