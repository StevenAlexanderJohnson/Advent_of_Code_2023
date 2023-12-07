use std::fs::read_to_string;

pub fn answer(file_path: String) -> i32 {
    let lines = match read_to_string(file_path) {
        Ok(input) => input.lines().map(String::from).collect::<Vec<String>>(),
        Err(_) => panic!("Could not read file"),
    };

    let mut output: i32 = 0;

    for line in lines {
        let mut calibration_value: String = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                calibration_value.push(c);
            }
        }

        if calibration_value.len() < 1 {
            continue;
        }

        let testing: String = format!(
            "{}{}",
            calibration_value.chars().nth(0).unwrap(),
            calibration_value.chars().nth_back(0).unwrap()
        );

        match testing.parse::<i32>() {
            Ok(value) => output += value,
            Err(_) => panic!("Could not parse value"),
        }
    }
    return output;
}
