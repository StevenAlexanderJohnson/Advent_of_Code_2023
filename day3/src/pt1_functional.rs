use std::fs::read_to_string;

pub fn answer(input_path: &str) -> Result<u32, &'static str> {
    let input = read_to_string(input_path).map_err(|_| "Error reading input file")?;

    let input_map: Vec<&str> = input.lines().collect();
    let width = match input_map.first() {
        Some(line) => line.len(),
        None => return Err("Empty input file"),
    };

    // Memory used for the dynamic programming solution
    let mut memory = vec![vec![false; width]; input_map.len()];

    // Indicies of all symbols in the map
    let indicies: Vec<(usize, usize)> = input_map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| !c.is_numeric() && c != '.')
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let mut output: u32 = 0;
    for index in indicies.iter() {
        output += get_numbers(&input_map, &mut memory, index)
            .map_err(|_| "Error collecting numbers")?
            .iter()
            .sum::<u32>();
    }

    Ok(output)
}

// Find all numbers around the specified index
fn get_numbers(
    input_map: &Vec<&str>,
    memory: &mut Vec<Vec<bool>>,
    index: &(usize, usize),
) -> Result<Vec<u32>, &'static str> {
    let mut output: Vec<u32> = vec![0; 0];

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

    for &(di, dj) in &DIRECTIONS {
        let x: i32 = index.0 as i32 + di;
        let y: i32 = index.1 as i32 + dj;

        // if out of bounds or already visited, skip
        if memory.get(x as usize).and_then(|row| row.get(y as usize)) == Some(&true) {
            continue;
        }

        // Get the character at the index, if it's a number (which it should be), get the numeric value of the whole number
        // and add to output.
        if let Some(c) = input_map
            .get(x as usize)
            .and_then(|line| line.chars().nth(y as usize))
        {
            if c.is_numeric() {
                match collect_number(input_map[x as usize], memory, (x as usize, y as usize)) {
                    Ok(number) => output.push(number),
                    Err(_) => return Err("Error collecting number"),
                }
            }
        }
    }

    Ok(output)
}

// Collect the whole number that touches the specified index
fn collect_number(
    input_map: &str,
    memory: &mut Vec<Vec<bool>>,
    index: (usize, usize),
) -> Result<u32, &'static str> {
    let chars: Vec<char> = input_map.chars().collect();
    let start_index = chars
        .iter()
        .enumerate()
        .take(index.1 + 1)
        .rposition(|(_, c)| !c.is_numeric())
        .map_or(0, |i| i + 1);

    let end_index: usize = chars
        .iter()
        .enumerate()
        .skip(index.1)
        .find(|(_, c)| !c.is_numeric())
        .map_or(chars.len(), |(i, _)| i);

    let number_str: String = input_map
        .chars()
        .skip(start_index)
        .take(end_index - start_index)
        .collect();

    for i in start_index..end_index {
        memory[index.0][i] = true;
    }

    number_str.parse().map_err(|_| "Error parsing number")
}
