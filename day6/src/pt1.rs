use crate::helper_functions::{find_race_error_margin, parse_input_into_races, read_input_file};

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let input: Vec<String> = match read_input_file(input_path) {
        Ok(file_data) => file_data,
        Err(e) => return Err(e),
    };

    if input.len() != 2 {
        return Err("Error when validating input file parse.");
    }

    let error_margin: u64 = parse_input_into_races(input)?
        .iter()
        .map(|race| find_race_error_margin(race))
        .fold(1, |x, y| x * y);

    Ok(error_margin)
}
