use std::fs::read_to_string;

pub struct RaceInfo {
    pub duration: u32,
    pub record: u32,
}

pub fn read_input_file(input_path: &str) -> Result<Vec<String>, &'static str> {
    Ok(read_to_string(input_path)
        .map_err(|_| "Unable to read input path.")?
        .lines()
        .filter_map(|line| {
            line.splitn(2, ' ')
                .skip(1)
                .next()
                .map(|s| s.trim().to_string())
        })
        .collect::<Vec<String>>())
}

pub fn parse_input_into_races(file_data: Vec<String>) -> Result<Vec<RaceInfo>, &'static str> {
    let race_records: Vec<u32> = file_data[1]
        .split_whitespace()
        .map(|race_record| {
            race_record
                .parse::<u32>()
                .map_err(|_| "Unable to parse race record.")
        })
        .collect::<Result<Vec<u32>, &'static str>>()?;

    // Parse the input strings into RaceInfo
    let races = file_data[0]
        .split_whitespace()
        .zip(race_records.into_iter())
        .map(|(duration, record)| {
            let duration: u32 = duration
                .parse::<u32>()
                .map_err(|_| "Unable to parse race duration.")?;
            Ok(RaceInfo {
                duration: duration,
                record: record,
            })
        })
        .collect::<Result<Vec<RaceInfo>, &'static str>>()?;

    Ok(races)
}

pub fn find_race_error_margin(race: &RaceInfo) -> u32 {
    let mut margin: u32 = 0;
    let mid: u32 = race.duration / 2;
    let mut charge_duration: u32 = mid;

    while (race.duration - charge_duration) * charge_duration > race.record {
        margin += 1;
        charge_duration -= 1;
    }

    charge_duration = mid + 1;
    while (race.duration - charge_duration) * charge_duration > race.record {
        margin += 1;
        charge_duration += 1;
    }

    margin
}
