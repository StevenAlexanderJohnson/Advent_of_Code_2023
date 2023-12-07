mod part1;
mod part2;
mod structs;
fn main() {
    match part1::answer("input.txt") {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e.to_string()),
    };

    match part2::answer("input.txt") {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e.to_string()),
    }
}
