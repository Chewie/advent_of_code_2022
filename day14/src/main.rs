use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (_, mut cave) = day14::Cave::parse(&input).unwrap();

    cave.step_until_abyss();

    println!("Number of rests before abyss: {}", cave.number_of_rests());

    let (_, mut cave_floor) = day14::Cave::parse(&input).unwrap();

    cave_floor.step_until_source_blocked();

    println!("Number of rests before source is blocked: {}", cave_floor.number_of_rests());
}
