use crate::helper_functions::{
    find_race_error_margin, read_input_file, remove_kerning_and_parse_race, RaceInfo,
};

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let input: Vec<String> = read_input_file(input_path)?;

    if input.len() != 2 {
        return Err("Error when validating input file parse.");
    }

    let race: RaceInfo = remove_kerning_and_parse_race(input)?;

    let error_margin: u64 = find_race_error_margin(&race);

    Ok(error_margin)
}
