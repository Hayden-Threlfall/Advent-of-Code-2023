use std::fs;
//use std::string;

fn main() {
    let str: String = fs::read_to_string("input.txt").expect("Error in reading the file");
    let mut count: i32 = 0;
    let lines = str.lines();
    for line in lines {
        for char in line.chars() {
            if char.is_numeric() {
                count += char.to_digit(10).unwrap() as i32 * 10;
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                count += char.to_digit(10).unwrap() as i32;
                break;
            }
            
        }
    }
    println!("{}", count);
}

