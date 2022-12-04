use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let worksheet = day04::WorkSheet::from_string(input).unwrap();
    let fully_count = worksheet.count_fully_contains();
    let overlaps = worksheet.count_overlaps();

    println!("Number of assignment pairs fully contained: {fully_count}");
    println!("Number of overlapping assignment pairs: {overlaps}");
}
