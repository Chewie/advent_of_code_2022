use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (_, signal) = day13::PairSignal::parse(&input).unwrap();

    println!("Sum of pairs in right order: {}", signal.sum_indices_in_right_order());

    let (_, signal) = day13::OrderedSignal::parse(&input).unwrap();

    println!("Decoder key: {}", signal.decoder_key());
}
