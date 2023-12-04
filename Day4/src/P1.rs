use std::{fs};

fn main() {
    let mut str1;
    let mut str2;
    let mut s;

    let mut total = 0;

    let f: String = fs::read_to_string("input.txt").expect("Error in reading the file");
    let lines = f.lines();
    let mut parts;
    let mut collection: Vec<&str>;
    
    for line in lines {
        let mut i = 0;
        let mut sum = 0;
        parts = line.split("|");
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
                        if sum == 0 {
                            sum = 1;
                            
                        }
                        else {
                            sum *= 2;
                        }
                    }
                }
            i += 1;
        }
        total += sum;  
    }
    println!("{}", total);
}