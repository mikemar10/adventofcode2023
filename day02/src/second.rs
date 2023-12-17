use day02::*;
fn main() {
    let input = include_str!("../input.txt");
    let sum: usize = input
        .split('\n')
        .take_while(|s| !s.is_empty())
        .map(|g| Game::parse_game(g).find_minimal_cube_power())
        .sum();
    println!("Answer: {}", sum);
}
