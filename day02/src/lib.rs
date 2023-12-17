#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub id: usize,
    pub draws: Vec<Draw>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Draw {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl Game {
    /// Given a string "game", returns a Game struct that contains the game ID and the draws made in that game
    ///
    /// A Draw is a random drawing of red, blue, or green cubes from the bag
    /// Cubes are returned to the bag after each draw
    /// A Game has an unsigned integer ID
    pub fn parse_game(game: &str) -> Self {
        let mut tokens = game.split_whitespace();
        let id = {
            let _ignored = tokens.next();
            tokens
                .next()
                .unwrap()
                .replace(':', "")
                .parse::<usize>()
                .unwrap()
        };
        let mut draws = vec![];
        let mut draw = Draw { r: 0, g: 0, b: 0 };
        while let (Some(n), Some(color)) = (tokens.next(), tokens.next()) {
            let n = n.parse::<usize>().unwrap();
            let mut new_draw = false;
            match color {
                "red," => draw.r = n,
                "green," => draw.g = n,
                "blue," => draw.b = n,
                "red;" | "red" => {
                    draw.r = n;
                    new_draw = true;
                }
                "green;" | "green" => {
                    draw.g = n;
                    new_draw = true;
                }
                "blue;" | "blue" => {
                    draw.b = n;
                    new_draw = true;
                }
                _ => unreachable!("This should never occur"),
            }
            if new_draw {
                draws.push(draw);
                draw = Draw { r: 0, g: 0, b: 0 };
            }
        }
        Self { id, draws }
    }

    /// Given a set of constraints for the number of r, g, b cubes in the bag, returns whether a game is possible
    /// based on the drawn values
    ///
    /// If any draw in the game is greater than the constraints provided, then the game is not possible
    pub fn is_game_possible(&self, r: usize, g: usize, b: usize) -> bool {
        for draw in &self.draws {
            if draw.r > r || draw.g > g || draw.b > b {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_game() {
        assert_eq!(
            Game::parse_game(
                "Game 49: 8 green, 13 blue, 3 red; 14 blue, 1 green; 14 blue, 2 green"
            ),
            Game {
                id: 49,
                draws: vec![
                    Draw { r: 3, g: 8, b: 13 },
                    Draw { r: 0, g: 1, b: 14 },
                    Draw { r: 0, g: 2, b: 14 }
                ]
            }
        );
    }

    fn test_is_game_possible() {
        let r = 12;
        let g = 13;
        let b = 14;
        let game1 = Game::parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game2 =
            Game::parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        let game3 = Game::parse_game(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        let game4 = Game::parse_game(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        );
        let game5 = Game::parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(game1.is_game_possible(r, g, b), true);
        assert_eq!(game2.is_game_possible(r, g, b), true);
        assert_eq!(game3.is_game_possible(r, g, b), false);
        assert_eq!(game4.is_game_possible(r, g, b), false);
        assert_eq!(game5.is_game_possible(r, g, b), true);
    }
}
