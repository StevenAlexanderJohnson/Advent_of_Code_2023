use std::fs::read_to_string;

pub fn read_file(input_path: &str) -> Result<Vec<String>, &'static str> {
    Ok(read_to_string(input_path)
        .map_err(|_| "Unable to read input file.")?
        .lines()
        .map(|lines| lines.trim().to_string())
        .collect::<Vec<String>>())
}

pub fn parse_numbers(input: &str) -> Result<Vec<u32>, &'static str> {
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
