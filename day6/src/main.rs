mod pt1;

fn main() {
    match pt1::answer("./day6_input/input.txt") {
        Ok(answer) => println!("Pt1. Answer: {}", answer),
        Err(e) => println!("{}", e)
    }
}
