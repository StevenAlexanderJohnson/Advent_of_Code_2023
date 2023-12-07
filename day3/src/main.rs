mod pt1;
mod pt1_functional;
mod pt2;

fn main() {
    match pt1::answer("./day3_input/input.txt") {
        Ok(output) => println!("{}", output),
        Err(e) => println!("{}", e),
    }

    match pt1_functional::answer("./day3_input/input.txt") {
        Ok(output) => println!("{}", output),
        Err(e) => println!("{}", e),
    }

    match pt2::answer("./day3_input/input.txt") {
        Ok(output) => println!("{}", output),
        Err(e) => println!("{}", e),
    }
}

