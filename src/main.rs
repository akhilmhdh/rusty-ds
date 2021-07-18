use std::io;

mod array;

fn main() {
    loop {
        let mut input = String::new();

        println!(
            "
-------------------------------------
Index
0. Exit
1. Array
-------------------------------------
Input:"
        );
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
