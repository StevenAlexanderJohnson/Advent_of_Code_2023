use std::collections::HashMap;

use crate::helper_functions;
use crate::structs::LotteryCard;

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let mut games: Vec<LotteryCard> = helper_functions::read_file(input_path)?
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() != 2 {
                return Err("Unable to separate game from info.");
            }

            let game_numbers: Vec<&str> = parts[0].split_whitespace().collect();
            if game_numbers.len() != 2 {
                return Err("Can't get card number");
            }
            let card_number: u32 = game_numbers[1]
                .parse::<u32>()
                .map_err(|_| "Unable to parse game number.")?;

            let game_parts: Vec<&str> = parts[1].splitn(2, " | ").collect();
            if game_parts.len() != 2 {
                return Err("Unable to seperate game numbers from player numbers");
            }
            let game_numbers = helper_functions::parse_numbers(game_parts[0])?;
            let player_numbers = helper_functions::parse_numbers(game_parts[1])?;

            Ok(LotteryCard {
                card_number: card_number,
                game_numbers: game_numbers,
                player_numbers: player_numbers,
                matches: 0,
            })
        })
        .collect::<Result<Vec<LotteryCard>, &'static str>>()?;

    for lottery_card in &mut games {
        let filtered: Vec<&u32> = lottery_card
            .player_numbers
            .iter()
            .filter(|game| lottery_card.game_numbers.contains(game))
            .collect();
        if !filtered.is_empty() {
            lottery_card.matches = filtered.len() as u32;
        }
    }

    let winning_games: Vec<&LotteryCard> = games
        .iter()
        .filter(|lottery_card| lottery_card.matches > 0)
        .collect();

    let mut copied_cards: HashMap<u32, u64> = HashMap::new();
    for card in &games {
        copied_cards.insert(card.card_number, 1);
    }

    for card in &winning_games {
        let copies = *copied_cards.get(&card.card_number).unwrap_or(&0);
        for i in 1..=card.matches {
            let next_card_number = card.card_number + i;
            if next_card_number <= games.len() as u32 {
                let entry = copied_cards.entry(next_card_number).or_insert(1);
                *entry += copies;
            }
        }
    }

    Ok(copied_cards.values().sum())
}
