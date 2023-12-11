use std::fs::read_to_string;

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let lines = match read_to_string(input_path) {
        Ok(lines) => lines,
        Err(_) => return Err("Error reading input file"),
    }
    .lines()
    .map(|line| line.trim().to_string())
    .collect::<Vec<String>>();

    let card_info: Vec<&str> = match lines
        .iter()
        .map(|line| {
            line.splitn(2, ": ")
                .nth(1)
                .ok_or("Unable to seperate game from info.")
        })
        .collect()
    {
        Ok(line) => line,
        Err(e) => {
            println!("Error in card_info");
            return Err(e);
        }
    };

    let game_info = match card_info
        .iter()
        .map(|game| game.splitn(2, " | "))
        .map(|mut split_str| {
            let first: Result<&str, &'static str> =
                split_str.next().ok_or("Unable to get game numbers");
            let second: Result<&str, &'static str> =
                split_str.next().ok_or("Unable to collect your numbers");
            first.and_then(|f| second.map(|s| (f, s)))
        })
        .collect::<Result<Vec<(&str, &str)>, &'static str>>()
    {
        Ok(info) => info,
        Err(e) => {
            println!("Error in game_info");
            return Err(e);
        }
    };

    let parsed_info = match game_info
        .iter()
        .map(|x| {
            let game_numbers = parse_numbers(x.0);
            let user_numbers = parse_numbers(x.1);
            game_numbers.and_then(|g| user_numbers.map(|u| (g, u)))
        })
        .collect::<Result<Vec<(Vec<u32>, Vec<u32>)>, &'static str>>()
    {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("Error in parsed_info");
            return Err(e);
        }
    };

    let matches: Vec<Vec<u32>> = parsed_info
        .iter()
        .map(|(first, second)| {
            second
                .iter()
                .filter(|item| first.contains(item))
                .cloned()
                .collect()
        })
        .collect();

    let output: u64 = matches
        .iter()
        .filter(|res| res.len() != 0)
        .map(|m| 1 * 2_u64.pow(m.len() as u32 - 1 as u32))
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
