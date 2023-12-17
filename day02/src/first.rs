use day02::*;
fn main() {
    let input = include_str!("../input.txt");
    let sum: usize = input
        .split('\n')
        .take_while(|s| !s.is_empty())
        .filter_map(|g| {
            let game = Game::parse_game(g);
            match game.is_game_possible(12, 13, 14) {
                true => Some(game.id),
                false => None,
            }
        })
        .sum();
    println!("Answer: {}", sum);
}
