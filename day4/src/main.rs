mod helper_functions;
mod pt1;
mod pt2;
mod structs;

fn main() {
    match pt1::answer("./day4_input/input.txt") {
        Ok(answer) => println!("Answer: {}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match pt2::answer("./day4_input/input.txt") {
        Ok(answer) => println!("Answer: {}", answer),
        Err(e) => println!("Error: {}", e),
    }
}
