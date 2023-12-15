use std::fs::read_to_string;

struct RaceInfo {
    duration: u32,
    record: u32,
}

pub fn answer(input_path: &str) -> Result<u32, &'static str> {
    let input: Vec<String> = match read_to_string(input_path) {
        Ok(input_file) => input_file
            .lines()
            // Trim to remove leading spaces if there were more than one spaces.
            .filter_map(|line| {
                line.splitn(2, ' ')
                    .skip(1)
                    .next()
                    .map(|s| s.trim().to_string())
            })
            .collect(),
        Err(_) => return Err("Unable to read input file."),
    };

    if input.len() != 2 {
        return Err("Error when validating input file parse.");
    }

    let race_records: Vec<u32> = input[1]
        .split_whitespace()
        .map(|race_record| {
            race_record
                .parse::<u32>()
                .map_err(|_| "Unable to parse race record.")
        })
        .collect::<Result<Vec<u32>, &'static str>>()?;

    // Parse the input strings into RaceInfo
    let races = input[0]
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

    let error_margin: u32 = races
        .iter()
        .map(|race| find_race_error_margin(race))
        .fold(1, |x, y| x * y);

    Ok(error_margin)
}

fn find_race_error_margin(race: &RaceInfo) -> u32 {
    let mut margin: u32 = 0;
    let mid: u32 = (2 * race.duration) / 4;
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

    println!("{}", margin);
    margin
}
