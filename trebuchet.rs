use std::fs::read_to_string;
use std::fs::File;

fn main() {
    println!("{}", compute("trebuchet-data"));
}

fn compute(filename: &str) -> u32 {
    let mut counter: u32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        counter += string_to_num(line);
    }

    counter
}

fn string_to_num(string: &str) -> u32 {
    let mut first_int: u32 = 0;
    let mut last_int: u32 = 0;

    for (index, char) in string.char_indices() {
        if char >= '1' && char <= '9' {
            last_int = char.to_digit(10).unwrap();
            if first_int == 0 {
                first_int = char.to_digit(10).unwrap();
            }
        }
    }

    first_int * 10 + last_int
}
