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
            line.splitn(2, ": ")
                .nth(1)
                .ok_or("Unable to seperate game from info.")
        })
        .map(|game| {
            let mut split_str = game?.splitn(2, " | ");
            let first = split_str.next().ok_or("Unable to get game numbers");
            let second = split_str.next().ok_or("Unable to get player numbers");

            first.and_then(|f| second.map(|s| (f, s)))
        })
        .map(|x| {
            let game_numbers = parse_numbers(x?.0);
            let user_numbers = parse_numbers(x?.1);
            game_numbers.and_then(|g| user_numbers.map(|u| (g, u)))
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
            s.trim().parse::<u32>().map_err(|e| {
                println!("{:?}", e);
                "Error parsing input"
            })
        })
        .collect()
}
