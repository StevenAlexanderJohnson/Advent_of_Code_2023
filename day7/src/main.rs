mod pt1;
fn main() {
    match pt1::answer("./day7_input/input.txt") {
        Ok(x) => println!("Pt1 answer: {}", x),
        Err(e) => println!("Pt1 error: {}", e),
    };
}
