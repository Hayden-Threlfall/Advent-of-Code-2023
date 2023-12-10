use std::{fs};

fn main() {
    part1();
}


fn part1() -> i32 {
    let input: String = fs::read_to_string("input.text").expect("Error in reading the file").to_string();
    let collection: Vec<_> = input.split("\n").collect();
    let directions = collection[0].chars();
    let mut index: usize;
    let mut next: String = "AAA".to_string();
    let mut count: i32 = 0;
    let mut d_Pos: usize = 0;
    //// Problem with my code is that im not jsut searching the first 3 letter, it is searching everthing as a whole and messing up because of that one way to fix is to split by it and seach all refreses for a \n -1 index away not good though
    while next != "ZZZ" {
        d_Pos += 1;
        if d_Pos >= directions.clone().count() {
            d_Pos = 0;
        }
        count += 1;
        index = input.find(&next).unwrap();
        println!("{}",index);
        if directions.clone().nth(d_Pos).unwrap() == 'L' {
            next = format!("{}{}{}",input.chars().nth(index+7).unwrap().to_string(),input.chars().nth(index+8).unwrap().to_string(),input.chars().nth(index+9).unwrap().to_string());
        }
        else {
            next = format!("{}{}{}",input.chars().nth(index+12).unwrap().to_string(),input.chars().nth(index+13).unwrap().to_string(),input.chars().nth(index+14).unwrap().to_string());
        }
        
    }
    
    println!("The output of the cards is...{}", count);
    return 0;
}
// Hash table lookup 