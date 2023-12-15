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
        return Err("Invalid input data.");
    }

    // Get the seed ranges
    let seeds = lines
        .iter()
        .nth(0)
        .ok_or("Unable to get first lines element.")?;

    let mut ranges = seeds
        .first()
        .ok_or("Unable to get seed values")?
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| {
            if chunk.len() != 2 {
                return Err("Invalid chunk size when parsing seeds.");
            }
            let first: u64 = chunk[0]
                .parse::<u64>()
                .map_err(|_| "Unable to parse seed range start.")?;
            let second: u64 = first
                + chunk[1]
                    .parse::<u64>()
                    .map_err(|_| "Unable to parse seed range end.")?;

            println!("{:?}", (first, second));
            Ok((first, second))
        })
        .collect::<Result<Vec<(u64, u64)>, &'static str>>()?;

    let transform_ranges = lines[1..]
        .iter()
        .map(|section| process_section(section))
        .collect::<Result<Vec<Vec<Transform>>, &'static str>>()?;

    for section in transform_ranges {
        for transform in section {
            println!(
                "Destination: {}, Source: {}-{}",
                transform.destination_range_start,
                transform.source_range_start,
                transform.source_range_start + transform.range
            );
            for range in &mut ranges {
                println!("Range: {:?}", range);
                // If the ranges overlap
                if !(range.1 < transform.source_range_start
                    || transform.source_range_start + transform.range < range.0)
                {
                    range.0 = range.0.max(transform.source_range_start);
                    range.1 = range.1.min(transform.source_range_start + transform.range);

                    range.0 = (range.0 - transform.source_range_start)
                        + transform.destination_range_start;
                    range.1 = (range.1 - transform.source_range_start)
                        + transform.destination_range_start;
                }
            }
        }
    }
    println!("{:?}", ranges);
    Ok(0)
}

fn process_section(lines: &Vec<&str>) -> Result<Vec<Transform>, &'static str> {
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
