use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut area = day12::Area::from_string(&input);

    let min_steps = area.min_steps();
    let min_steps_all = area.min_steps_from_all_a();

    println!("Minimum steps: {min_steps}");
    println!("Minimum steps from all a: {min_steps_all}");
}
