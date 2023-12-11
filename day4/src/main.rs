mod pt1;

fn main() {
    match pt1::answer("./day4_input/input.txt") {
        Ok(answer) => println!("Answer: {}", answer),
        Err(e) => println!("Error: {}", e),
    }
}
