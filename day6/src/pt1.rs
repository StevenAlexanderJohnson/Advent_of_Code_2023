use crate::helper_functions::{find_race_error_margin, parse_input_into_races, read_input_file};

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let error_margin = read_input_file(input_path)
        .and_then(|input| {
            if input.len() != 2 {
                Err("Error when validating input file parse.")
            } else {
                Ok(input)
            }
        })
        .and_then(|x| parse_input_into_races(x))?
        .iter()
        .map(|race| find_race_error_margin(race))
        .fold(1, |x, y| x * y);

    Ok(error_margin)
}
