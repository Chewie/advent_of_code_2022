use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let worksheet = day04::WorkSheet::from_string(input).unwrap();
    let count = worksheet.count_fully_contains();

    println!("Number of overlapping assignments: {count}");
}
