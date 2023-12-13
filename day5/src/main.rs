mod pt1;
fn main() {
    match pt1::answer("./day5_input/input.txt") {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
