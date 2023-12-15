use crate::helper_functions::{
    find_race_error_margin, read_input_file, remove_kerning_and_parse_race,
};

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let error_margin: u64 = read_input_file(input_path)
        .and_then(|input| {
            if input.len() != 2 {
                Err("Error when validating input file parse.")
            } else {
                remove_kerning_and_parse_race(input)
            }
        })
        .and_then(|race| Ok(find_race_error_margin(&race)))?;

    Ok(error_margin)
}
