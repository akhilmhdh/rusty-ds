use std::io;

use crate::{util_formaters::Formaters, utils::read_a_number};

pub fn main() {
    Formaters::title("VECTOR");

    Formaters::subtitle(
        "Vectors are variable dimension array. Its stored in heap with a pointer in register.",
    );

    Formaters::text(
        "Below is an empty vector and created by 
let vector: Vec<i32> = []",
    );
    let mut vector: Vec<i32> = Vec::new();

    println!("Original array: {:?}", vector);

    loop {
        let mut input = String::new();
        Formaters::text(
            "Select a vector operation:
0. Go back to index
1. Display the vector
2. Push an element
3. Pop an element
4. Insert an element from an index
5. Update an element from an index
6. Remove an element from an index
7. Display length
9. Other operations
",
        );
        io::stdin()
            .read_line(&mut input)
            .expect("Operation number must be given");

        let input: i32 = input.trim().parse().expect("To be a number");

        match input {
            0 => break,
            1 => {
                println!("Vector: {:?}", vector);
            }
            2 => {
                Formaters::text("To push an element: vector.push(<value>)");
                let pushed_element = read_a_number("to be pushed");
                vector.push(pushed_element)
            }
            3 => {
                Formaters::text("To pop an element: vector.pop()");
                let popped_element = vector.pop();
                match popped_element {
                    Some(x) => println!("{}", x),
                    None => println!("Vector is empty"),
                }
            }
            4 => {
                Formaters::text(
                    "To insert an element from an index: vector.insert(<index>,value))",
                );
                let index = read_a_number("for index") as usize;
                if vector.len() < index {
                    println!("Out of bounds");
                    continue;
                }
                let new_element = read_a_number("to be inserted");
                vector.insert(index, new_element);
            }
            5 => {
                Formaters::text(
                    "Vector index can be used to modify data at an index. vector[<index>] = value",
                );
                let index = read_a_number("for index") as usize;
                if vector.len() < index {
                    println!("Out of bounds");
                    continue;
                }
                let new_element = read_a_number("to replace");
                vector[index] = new_element;
            }
            6 => {
                Formaters::text("To remove an element from an index: vector.remove(<index>)");
                let index = read_a_number("for index") as usize;
                let removed_element = vector.remove(index);
                println!("Removed Element: {}", removed_element);
            }
            7 => {
                Formaters::text("To get length of a vector: vector.len()");
                println!("Length is: {}", vector.len());
            }
            8 => {
                Formaters::text("To clear a vector: vector.clear()");
                vector.clear();
                println!("Vector: {:?}", vector);
            }
            9 => {
                Formaters::subtitle("Some of the common other operations");
                Formaters::text("vector.is_empty to check vector is empty or not");
                println!("Vector is empty: {}", vector.is_empty());

                Formaters::text("There are other functions like splice, resize. For more visit https://doc.rust-lang.org/std/vec/struct.Vec.html");
            }
            _ => println!("invalid option"),
        }
    }
}
