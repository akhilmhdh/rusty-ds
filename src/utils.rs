use std::{io, panic};

pub fn read_a_number(number_type: &str) -> i32 {
    let mut input_number = String::new();
    println!("Enter a number {} >>", number_type);
    io::stdin()
        .read_line(&mut input_number)
        .unwrap_or_else(|_| panic!("{} must be given", number_type));

    let input: i32 = input_number.trim().parse().expect("To be a number");
    return input;
}
