use crate::util_formaters::Formaters;

pub fn main() {
    Formaters::title("ARRAY");

    Formaters::subtitle(
        "Array's are fixed dimension same value DS in rust, and must be given in compile time.
For a variable one check out vector",
    );

    Formaters::text(
        "Below is an array of dimension 5 and created by 
let mut array: [<type>; <array_size>] = [<initial_value>;<fill_number>];",
    );
    let mut array: [i32; 5] = [0, 1, 2, 3, 4];

    println!("Original array: {:?}", array);

    Formaters::text("To update an array value, eg: array[2]=10");

    array[2] = 10;

    println!("Updated array:{:?}", array);

    Formaters::text(
        "To iterate we use for-loop with .iter() and to get index with that .iter().enumerate()",
    );

    for (index, el) in array.iter().enumerate() {
        println!("- array position is {} and value is {}", index, el);
    }
}
