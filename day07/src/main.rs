use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let fs = day07::FileSystem::from_string(&input);
    let size = fs.total_size_under_100k();

    println!("Total size under 100k : {size}");
}
