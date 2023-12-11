use std::fs::read_to_string;

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let lines = read_to_string(input_path)
        .map_err(|_| "Unable to read input file.")?
        .lines()
        .map(|lines| lines.trim().to_string())
        .collect::<Vec<String>>();

    let card_info: Vec<(Vec<u32>, Vec<u32>)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() != 2 {
                return Err("Unable to separate game from info.");
            }
            let game_parts: Vec<&str> = parts[1].splitn(2, " | ").collect();
            if game_parts.len() != 2 {
                return Err("Unable to seperate game numbers from player numbers");
            }
            let game_numbers = parse_numbers(game_parts[0])?;
            let user_numbers = parse_numbers(game_parts[1])?;
            Ok((game_numbers, user_numbers))
        })
        .collect::<Result<Vec<(Vec<u32>, Vec<u32>)>, &'static str>>()?;

    let output: u64 = card_info
        .iter()
        .filter_map(|(first, second)| {
            let filtered: Vec<&u32> = second.iter().filter(|item| first.contains(item)).collect();
            if !filtered.is_empty() {
                Some(1 * 2_u64.pow(filtered.len() as u32 - 1))
            } else {
                None
            }
        })
        .sum();

    Ok(output)
}

fn parse_numbers(input: &str) -> Result<Vec<u32>, &'static str> {
    input
        .split_whitespace()
        .map(|s| {
            s.parse::<u32>().map_err(|e| {
                eprintln!("{:?}", e);
                "Error parsing input"
            })
        })
        .collect()
}
