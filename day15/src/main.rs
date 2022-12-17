use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let tunnel = day15::Tunnel::parse(&input, 4000000, 4000000);

    //println!(
    //"Number of not beacons on line 2000000: {}",
    //tunnel.number_of_not_beacons_on_line(2000000)
    //);

    println!("Tuning frequency: {}", tunnel.tuning_frequency());
}
