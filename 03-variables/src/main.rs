use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index = index
        .trim()
        .parse::<usize>()
        .expect("Not a valid number please try again");

    let element = a[index];

    println!("The indexed element at position {} is {}", index, element)
}
