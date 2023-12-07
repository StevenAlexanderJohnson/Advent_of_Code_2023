use std::fs::read_to_string;

pub fn answer(input_path: &str) -> Result<u32, &'static str> {
    let input = read_to_string(input_path).map_err(|_| "Error reading input file")?;

    let input_map: Vec<String> = input.lines().map(|string| string.to_owned()).collect();

    // Get the indicies of all astrisks
    let indicies: Vec<(usize, usize)> = input_map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '*')
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let sums = indicies
        .iter()
        .map(|index| touches_two_numbers(&input_map, index))
        .collect::<Result<Vec<u32>, &'static str>>()?;

    Ok(sums.iter().sum())
}

fn touches_two_numbers(
    input_map: &Vec<String>,
    index: &(usize, usize),
) -> Result<u32, &'static str> {
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // Create the memory for the dynamic programming solution
    let width: usize = match input_map.first() {
        Some(line) => line.len(),
        None => Err("Empty input file")?,
    };
    let mut memory = vec![vec![false; width]; input_map.len()];

    let mut numbers: Vec<u32> = Vec::new();
    // Check the surrounding indicies for numbers
    let mut count = 0;
    for &(di, dj) in &DIRECTIONS {
        let x: i32 = index.0 as i32 + di;
        let y: i32 = index.1 as i32 + dj;

        // If already checked, skip
        if memory.get(x as usize).and_then(|row| row.get(y as usize)) == Some(&true) {
            continue;
        }

        // If out of bounds, skip
        // Shouldn't because we already checked memory which is the same shape
        let character: char = match input_map
            .get(x as usize)
            .and_then(|row| row.chars().nth(y as usize))
        {
            Some(c) => c,
            None => continue,
        };

        if character.is_numeric() {
            // If this is the second number, then return
            count += 1;
            // Mark the whole number as visited
            match touch_number(
                &input_map[x as usize],
                &mut memory,
                &(x as usize, y as usize),
            ) {
                Ok(number) => numbers.push(number),
                Err(e) => return Err(e),
            }
            if count == 2 {
                return Ok(numbers.iter().fold(1, |acc, x| acc * x));
            }
        }
    }

    // Return 0 because we are going to sum all at the end, and we don't want to sum None
    Ok(0)
}

fn touch_number(
    input: &String,
    memory: &mut Vec<Vec<bool>>,
    index: &(usize, usize),
) -> Result<u32, &'static str> {
    let chars: Vec<char> = input.chars().collect();

    let start_index = chars
        .iter()
        .enumerate()
        .take(index.1 + 1)
        .rposition(|(_, c)| !c.is_numeric())
        .map_or(0, |i| i + 1);

    let end_index = chars
        .iter()
        .enumerate()
        .skip(index.1)
        .find(|(_, c)| !c.is_numeric())
        .map_or(chars.len(), |(i, _)| i);

    (start_index..end_index).for_each(|i| memory[index.0][i] = true);

    println!(
        "{}",
        input
            .chars()
            .skip(start_index)
            .take(end_index - start_index)
            .collect::<String>()
    );

    input
        .chars()
        .skip(start_index)
        .take(end_index - start_index)
        .collect::<String>()
        .parse::<u32>()
        .map_err(|_| "Error parsing number")
}
