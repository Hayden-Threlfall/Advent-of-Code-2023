use std::{fs, vec};
fn main() {
    let str: String = fs::read_to_string("input.txt").expect("Error in reading the file");
    let vec = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut count: i32 = 0;
    let mut low: i32 = 0;
    let mut high: i32 = 0;
    let mut posL: usize = 1000;
    let mut posH: usize = 0;
    //let mut count: i32 = 0;
    let lines = str.lines();
    for line in lines {
        for char in line.chars() {
            if char.is_numeric() {
                    low = char.to_digit(10).unwrap() as i32;
                    posL = line.find(char).unwrap();
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                    high = char.to_digit(10).unwrap() as i32;
                    posH = line.rfind(char).unwrap();
                break;
            }
        }
        for i in 0..vec.len() {
            if line.find(vec[i]).unwrap_or(1000) < posL {
                low = i as i32 + 1;
                posL = line.find(vec[i]).unwrap();
            }
            if line.rfind(vec[i]).unwrap_or(0) > posH {
                high = i as i32 + 1;
                posH = line.rfind(vec[i]).unwrap();
            }
        }
        count += (low * 10) + high;
        low = 0;
        high = 0;
        posL = 1000;
        posH = 0;

    }
    println!("{}", count);
}