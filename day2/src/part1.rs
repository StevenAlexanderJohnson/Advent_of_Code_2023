use crate::structs::game::Game;
use std::fs::read_to_string;

pub fn answer(input_file: &str) -> Result<i32, String> {
    let mut output: i32 = 0;
    let lines = match read_to_string(input_file) {
        Ok(input) => input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        Err(e) => return Err(e.to_string()),
    };

    for line in lines {
        let game: Game = match Game::from_string(line) {
            Ok(game) => game,
            Err(e) => return Err(e.to_string()),
        };
        if !game.validate(12, 13, 14) {
            continue;
        }
        output += game.id;
    }

    Ok(output)
}
