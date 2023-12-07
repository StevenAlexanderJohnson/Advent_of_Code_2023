use std::{collections::HashMap, fs::read_to_string};

pub fn answer(file_path: String) -> i32 {
    let lines: Vec<String> = match read_to_string(file_path) {
        Ok(input) => input.lines().map(String::from).collect::<Vec<String>>(),
        Err(_) => panic!("Could not read file")
    };

    let output: i32 = 0;

    for line in lines {
        let _ = sliding_window(&line);
    }
    return output;
}

fn sliding_window(line: &str) -> String {
    let int_strings: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);
    let output: String = String::new();

    let mut start: usize = 0;
    while start + 5 < line.len() {
        let line: String = line.chars().skip(start).take(5).collect();
        for key in int_strings.keys() {
            if line.contains(key) {
                println!("{}", line);
            }
        }
        start += 1;
    }

    output
}
