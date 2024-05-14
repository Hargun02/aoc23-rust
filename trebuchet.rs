use std::fs::read_to_string;

fn main() {
    println!("{}", compute("trebuchet-data", true));
}

fn compute(filename: &str, part_2 : bool) -> u32 {
    let mut counter: u32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
      if !part_2{
        counter += string_to_num(line);
      }
      else {
        let string = line.replace("one", "1").replace("two", "2")
          .replace("three", "3").replace("four", "4").replace("five", "5")
          .replace("six", "6").replace("seven", "7").replace("eight", "8")
          .replace("nine", "9");
        println!("{}", string);
        counter += string_to_num(&string);
      }
    }

    counter
}

fn string_to_num(string: &str) -> u32 {
    let mut first_int: u32 = 0;
    let mut last_int: u32 = 0;

    for (_index, char) in string.char_indices() {
        if char >= '1' && char <= '9' {
            last_int = char.to_digit(10).unwrap();
            if first_int == 0 {
                first_int = char.to_digit(10).unwrap();
            }
        }
    }

    first_int * 10 + last_int
}
