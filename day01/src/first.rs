use day01::*;
fn main() {
    let input = include_str!("../input.txt");
    let sum = input
        .lines()
        .map(extract_number)
        .reduce(|acc, e| acc + e)
        .expect("There should not be an input that triggers this");
    println!("Answer: {}", sum);
}
