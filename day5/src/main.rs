mod pt1;
fn main() {
    match pt1::answer("./day5_input/test_input.txt") {
        Ok(x) => println!("Pt1 Answer: {}", x),
        Err(e) => println!("{}", e),
    }
}
