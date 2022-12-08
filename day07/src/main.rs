use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let fs = day07::FileSystem::from_string(&input);
    let size_under_100k = fs.total_size_under_100k();
    let smallest_dir = fs.smallest_dir_big_enough();

    println!("Total size under 100k : {size_under_100k}");
    println!("Size of smallest dir that can be removed: {smallest_dir}");
}
