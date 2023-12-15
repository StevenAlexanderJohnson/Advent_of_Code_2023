use std::fs::read_to_string;

#[derive(Debug)]
struct Transform {
    source_range_start: u64,
    destination_range_start: u64,
    range: u64,
}

pub fn answer(input_path: &str) -> Result<u64, &'static str> {
    let input = read_to_string(input_path).map_err(|_| "Unable to read input file.")?;

    let lines: Vec<Vec<&str>> = input
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.trim().is_empty())
        .map(|section| section.to_vec())
        .collect();

    if lines.len() != 8 {
        return Err("The data is not formatted correctly.");
    }

    let mut lines_iter = lines.into_iter();

    let first_line = lines_iter.next().ok_or("No lines in the input.")?;
    if first_line.len() != 1 {
        return Err("The first line should only contain one element.");
    }

    let mut seeds: Vec<u64> = first_line[0]
        .split_whitespace()
        .map(|x| x.parse::<u64>().map_err(|_| "Unable to parse number"))
        .collect::<Result<Vec<u64>, &'static str>>()?;

    while let Some(line) = lines_iter.next() {
        let transform = process_section(line)?;
        perform_transformation(&mut seeds, transform)?;
    }

    let min_value: u64 = match seeds.iter().min() {
        Some(x) => *x,
        None => return Err("Unable to get minimum value."),
    };

    Ok(min_value)
}

fn process_section(lines: Vec<&str>) -> Result<Vec<Transform>, &'static str> {
    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                return Err("Unable to parse line into range.");
            }

            Ok(Transform {
                destination_range_start: parts[0]
                    .parse::<u64>()
                    .map_err(|_| "Unable to parse destination start to u64")?,
                source_range_start: parts[1]
                    .parse::<u64>()
                    .map_err(|_| "Unable to parse source start to u64")?,
                range: parts[2]
                    .parse::<u64>()
                    .map_err(|_| "Unableto parse range to u64")?,
            })
        })
        .collect::<Result<Vec<Transform>, &'static str>>()
}

fn perform_transformation(
    seeds: &mut Vec<u64>,
    transformations: Vec<Transform>,
) -> Result<(), &'static str> {
    let transform = |x: u64| -> u64 {
        for t in &transformations {
            if t.source_range_start <= x && t.source_range_start+t.range >=x {
                return t.destination_range_start + (x - t.source_range_start);
            }
        }
        x
    };

    seeds.iter_mut().for_each(|seed| *seed = transform(*seed));

    Ok(())
}
