use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let graph = day16::Cave::parse(&input);

    println!("Max pressure after 30mins: {}", graph.max_pressure());

    println!("Max pressure with elephant after 26mins: {}", graph.max_pressure_with_elephant());
}
