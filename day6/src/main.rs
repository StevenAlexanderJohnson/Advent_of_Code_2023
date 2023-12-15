mod helper_functions;
mod pt1;
mod pt2;

fn main() {
    match pt1::answer("./day6_input/input.txt") {
        Ok(answer) => println!("Pt1. Answer: {}", answer),
        Err(e) => println!("{}", e),
    }

    match pt2::answer("./day6_input/input.txt") {
        Ok(answer) => println!("Pt2. Answer: {}", answer),
        Err(e) => println!("{}", e),
    }
}
