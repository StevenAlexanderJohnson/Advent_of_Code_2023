use std::fs::read_to_string;

pub struct RaceInfo {
    pub duration: u64,
    pub record: u64,
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
    let race_records: Vec<u64> = file_data[1]
        .split_whitespace()
        .map(|race_record| {
            race_record
                .parse::<u64>()
                .map_err(|_| "Unable to parse race record.")
        })
        .collect::<Result<Vec<u64>, &'static str>>()?;

    // Parse the input strings into RaceInfo
    let races = file_data[0]
        .split_whitespace()
        .zip(race_records.into_iter())
        .map(|(duration, record)| {
            let duration: u64 = duration
                .parse::<u64>()
                .map_err(|_| "Unable to parse race duration.")?;
            Ok(RaceInfo {
                duration: duration,
                record: record,
            })
        })
        .collect::<Result<Vec<RaceInfo>, &'static str>>()?;

    Ok(races)
}

pub fn remove_kerning_and_parse_race(file_data: Vec<String>) -> Result<RaceInfo, &'static str> {
    let race_record: String = str::replace(&file_data[1], " ", "");

    let race_record = race_record
        .parse::<u64>()
        .map_err(|_| "Unable to parse race record.")?;

    let race_duration: u64 = str::replace(&file_data[0], " ", "")
        .parse::<u64>()
        .map_err(|_| "Unable to parse race duration.")?;

    Ok(RaceInfo {
        record: race_record,
        duration: race_duration,
    })
}

// If we look at the pattern for the test input we can see that the distance traveled is a quadratic equation.
// We can see that the race duration is the constant, and charge time is the variable.
// distance = (race_duration - charge_time) * charge_time == (k - x) * x == -x^2 + kx
// There is an equation that describes the max point of a quadratic equation, but seeing it always starts at 0 and we have a max value,
// the max value is always the middle of the domain.
pub fn find_race_error_margin(race: &RaceInfo) -> u64 {
    let mut margin: u64 = 0;
    let mid: u64 = race.duration / 2;
    let mut charge_duration: u64 = mid;

    // Iterate because quadratic equation requires negative numbers which u64 doesn't allow.
    // We already know the max range, so iteration won't take that much longer.
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
