mod pt1;
mod pt2;
fn main() {
    match pt1::answer("./day5_input/input.txt") {
        Ok(x) => println!("Pt1 Answer: {}", x),
        Err(e) => println!("{}", e),
    }

    // Day 2 is incorrect.
    match pt2::answer("./day5_input/test_input.txt") {
        Ok(x) => println!("Pt2 Answer: {}", x),
        Err(e) => println!("{}", e)
    }
}
