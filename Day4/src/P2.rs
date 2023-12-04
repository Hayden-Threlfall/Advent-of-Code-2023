use std::{fs};

fn main() {
    let mut str1;
    let mut str2;
    let mut s;
    let mut card = 0;
    let mut total = 0;

    let f: String = fs::read_to_string("input.txt").expect("Error in reading the file");
    let lines = f.lines();
    let length = f.matches("\n").count()+1;

    let mut arr: Vec<i32> = vec![0; length];
    let mut parts;
    let mut collection: Vec<&str>;
    
    for _line in lines {
        let mut i = 0;
        let mut sum = 0;
        parts = _line.split("|");
        collection = parts.collect();
        str2 = collection[1].to_string() + " ";

        parts =  collection[0].split(": ");
        collection = parts.collect();
        str1 = collection[1];
        
            while i<str1.len(){
                if str1.chars().nth(i).unwrap().is_numeric() {
                    s = str1.chars().nth(i).unwrap().to_string();
                    i += 1;
                    if str1.chars().nth(i).unwrap().is_numeric(){
                        s = s + &str1.chars().nth(i).unwrap().to_string();
                    }
                        s =  format!(" {s} ");
                    if str2.contains(&s) {
                        sum += 1;
                    }
                }
            i += 1;
        }
        arr[card] += 1;
        for _p in 0..arr[card] {
            for x in card+1..card+sum+1 {
                arr[x] += 1;
            }
            
        }
        card += 1;
    }
    for x in arr {
        total += x;
    }
    println!("{}", total);
}