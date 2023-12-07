use std::fs::read_to_string;

pub fn answer(input_path: &str) -> Result<u32, &'static str> {
    let mut output: u32 = 0;

    let input = read_to_string(input_path).map_err(|_| "Error reading input file")?;

    let input_map: Vec<&str> = input.lines().collect();
    let width = match input_map.first() {
        Some(line) => line.len(),
        None => return Err("Empty input file"),
    };

    // Memory used for the dynamic programming solution
    let mut memory = vec![vec![false; width]; input_map.len()];

    // Find the symbols in the map
    for (i, line) in input_map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() || c == '.' {
                continue;
            }
            output += get_numbers(&input_map, &mut memory, (i, j))
                .iter()
                .sum::<u32>();
        }
    }

    Ok(output)
}

// Find all numbers around the specified index
fn get_numbers(
    input_map: &Vec<&str>,
    memory: &mut Vec<Vec<bool>>,
    index: (usize, usize),
) -> Vec<u32> {
    let mut output: Vec<u32> = vec![0; 0];

    // Check all 8 directions
    for i in -1..=1 as i32 {
        for j in -1..=1 as i32 {
            if i == 0 && j == 0 {
                continue;
            }
            let x: i32 = index.0 as i32 + i;
            let y: i32 = index.1 as i32 + j;

            // already visited
            if memory[x as usize][y as usize] {
                continue;
            }

            // out of bounds
            if x < 0 || y < 0 || x >= input_map.len() as i32 || y >= input_map[0].len() as i32 {
                continue;
            }
            let line = match input_map.get(x as usize) {
                Some(line) => line,
                None => continue,
            };
            let c = match line.chars().nth(y as usize) {
                Some(c) => c,
                None => continue,
            };

            if c.is_numeric() {
                output.push(collect_number(input_map, memory, (x as usize, y as usize)))
            }
        }
    }

    output
}

// Collect the whole number that touches the specified index
fn collect_number(
    input_map: &Vec<&str>,
    memory: &mut Vec<Vec<bool>>,
    index: (usize, usize),
) -> u32 {
    let mut horizontal: usize = index.1;
    while horizontal > 0
        && input_map[index.0]
            .chars()
            .nth(horizontal - 1)
            .unwrap()
            .is_numeric()
    {
        horizontal -= 1;
    }

    let mut length = 0;
    for i in horizontal..input_map[index.0].len() {
        if !input_map[index.0].chars().nth(i).unwrap().is_numeric() {
            break;
        }
        memory[index.0][i] = true;
        length += 1;
    }
    let number: String = input_map[index.0]
        .chars()
        .skip(horizontal)
        .take(length)
        .collect();

    match number.parse::<u32>() {
        Ok(number) => number,
        Err(_) => panic!("Error parsing number: {}", number),
    }
}
