use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let sop_idx = day06::index_after_start_of_packet(&input);
    let som_idx = day06::index_after_start_of_message(&input);

    println!("Index after start of packet: {sop_idx}");
    println!("Index after start of message: {som_idx}");
}
