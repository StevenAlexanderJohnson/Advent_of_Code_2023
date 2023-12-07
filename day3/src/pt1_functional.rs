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

    let sum: u32 = indicies
        .iter()
        .map(|index| {
            get_numbers(&input_map, &mut memory, index)
                .iter()
                .sum::<u32>()
        })
        .sum();

    Ok(sum)
}

// Find all numbers around the specified index
fn get_numbers(
    input_map: &Vec<&str>,
    memory: &mut Vec<Vec<bool>>,
    index: &(usize, usize),
) -> Vec<u32> {
    let mut output: Vec<u32> = vec![0; 0];

    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for &(di, dj) in &directions {
        let x: i32 = index.0 as i32 + di;
        let y: i32 = index.1 as i32 + dj;

        if x < 0 || y < 0 || x >= input_map.len() as i32 || y >= input_map[0].len() as i32 {
            continue;
        }
        if memory[x as usize][y as usize] {
            continue;
        }

        if let Some(c) = input_map
            .get(x as usize)
            .and_then(|line| line.chars().nth(y as usize))
        {
            if c.is_numeric() {
                memory[x as usize][y as usize] = true;
                output.push(collect_number(
                    input_map[x as usize],
                    memory,
                    (x as usize, y as usize),
                ))
            }
        }
    }

    output
}

// Collect the whole number that touches the specified index
fn collect_number(input_map: &str, memory: &mut Vec<Vec<bool>>, index: (usize, usize)) -> u32 {
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

    match number_str.parse() {
        Ok(number) => number,
        Err(_) => {
            panic!("Error parsing number: {}", number_str); // panic becuase the rest of the logic is wrong.
        }
    }
}
