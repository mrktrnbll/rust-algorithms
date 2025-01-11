use std::fs;

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    return contents;
}

fn string_to_array(string: String) -> Vec<i32> {
    let numbers = string.split(" ").map(|number| number.parse::<i32>().unwrap()).map(|number| number).collect();

    return numbers;
}

fn main() {
    println!("Hello, world!");

    println!("The file contains: {}", read_file("data/100.txt"));
    let string = read_file("data/100.txt");
    let array = string_to_array(string);
    println!("{:?}", array);
}
