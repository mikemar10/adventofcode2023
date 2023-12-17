use day02::*;
fn main() {
    let input = include_str!("../input.txt");
    let sum = input
        .split('\n')
        .take_while(|s| !s.is_empty())
        .map(Game::parse_game)
        .filter(|game| game.is_game_possible(12, 13, 14))
        .fold(0, |acc, game| acc + game.id);
    println!("Answer: {}", sum);
}
