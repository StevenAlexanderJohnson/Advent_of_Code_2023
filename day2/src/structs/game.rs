pub struct Game {
    pub id: i32,
    pub rounds: Vec<Round>,
}

pub struct Round {
    red_cubes: i32,
    blue_cubes: i32,
    green_cubes: i32,
}

impl Game {
    pub fn validate(&self, max_red_count: i32, max_green_count: i32, max_blue_count: i32) -> bool {
        self.rounds.iter().all(|x| {
            x.red_cubes <= max_red_count
                && x.blue_cubes <= max_blue_count
                && x.green_cubes <= max_green_count
        })
    }

    pub fn calculate_power(&self) -> i64 {
        let (red_cubes, blue_cubes, green_cubes) =
            self.rounds
                .iter()
                .fold((0, 0, 0), |(red_min, blue_min, green_min), round| {
                    (
                        red_min.max(round.red_cubes as i64),
                        blue_min.max(round.blue_cubes as i64),
                        green_min.max(round.green_cubes as i64),
                    )
                });
        red_cubes * blue_cubes * green_cubes
    }

    pub fn from_string(game_line: String) -> Result<Self, &'static str> {
        // game_line should look like this: "Game 1: 1 red; 2 blue 3 green; 3 green"
        let mut game_definition_parts = game_line.splitn(2, ':');

        let game_id: i32 = game_definition_parts // ["Game 1", " 1 red; 2 blue, 3 green; 3 green"]
            .next() // "Game 1"
            .and_then(|x| x.trim().split_whitespace().next_back()) // "1"
            .and_then(|x| x.parse::<i32>().ok()) // 1
            .ok_or_else(|| "Unable to parse game ID to i32.")?;

        let mut output_game = Game {
            id: game_id,
            rounds: vec![],
        };

        game_definition_parts
            .next() // "1 red; 2 blue, 3 green; 3 green"
            .ok_or_else(|| "Unable to parse find the game ID")?
            .split(';') // ["1 red", "2 blue, 3 green", "3 green"]
            .for_each(|round_str| {
                let mut round = Round {
                    red_cubes: 0,
                    blue_cubes: 0,
                    green_cubes: 0,
                };

                round_str.split(',').for_each(|x| {
                    let mut round_parts = x.trim().split_whitespace();

                    let cubes = match round_parts.next().and_then(|x| x.parse::<i32>().ok()) {
                        Some(cubes) => cubes,
                        None => return,
                    };

                    let color = round_parts.next();

                    match color {
                        Some("red") => round.red_cubes += cubes,
                        Some("blue") => round.blue_cubes += cubes,
                        Some("green") => round.green_cubes += cubes,
                        _ => panic!("Invalid color"),
                    }
                });
                output_game.rounds.push(round);
            });

        Ok(output_game)
    }
}
