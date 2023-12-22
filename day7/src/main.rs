mod pt1;
fn main() {
    println!("Hello, world!");
    match pt1::answer("./day7_input/test_input.txt") {
        Ok(x) => println!("Pt1 answer: {}", x),
        Err(e) => println!("Pt1 error: {}", e),
    };
}
