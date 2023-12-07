mod pt1;
mod pt1_functional;

fn main() {
    match pt1::answer("input.txt") {
        Ok(output) => println!("{}", output),
        Err(e) => println!("{}", e),
    }

    match pt1_functional::answer("input.txt") {
        Ok(output) => println!("{}", output),
        Err(e) => println!("{}", e),
    }
}
