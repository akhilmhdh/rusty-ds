use std::io;

mod array;
mod util_formaters;

use util_formaters::Formaters;

fn main() {
    loop {
        let mut input = String::new();
        let titles = ["Exit", "Array"];

        Formaters::appendix(&titles);
        io::stdin()
            .read_line(&mut input)
            .expect("DSA number must be given");

        let input: i32 = input.trim().parse().expect("To be a number");
        match input {
            0 => break,
            1 => array::main(),
            _ => println!("invalid option"),
        }
    }
}
