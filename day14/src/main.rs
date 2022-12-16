use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (_, mut cave) = day14::Cave::parse(&input).unwrap();

    cave.step_until_abyss();

    println!("Number of rests before abyss: {}", cave.number_of_rests());
}
