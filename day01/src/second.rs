use day01::*;
fn main() {
    let input = include_str!("../input.txt");
    let sum = input
        .split('\n')
        .map(extract_number6)
        .reduce(|acc, e| acc + e)
        .expect("There should not be an input that triggers this");
    println!("Answer: {}", sum);
}
