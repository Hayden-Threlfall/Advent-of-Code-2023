use std::{fs};
use std::collections::HashMap;
fn main() {
    // Parser reading file as string
    let input: String = fs::read_to_string("input.text").expect("Error in reading the file").to_string();
    //part1(input);
    part2(input);
}


fn part1(input: String) {
    let mut lines = input.lines();
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let collection = lines.next().unwrap();
    let directions: Vec<char> = collection.chars().collect();
    lines.next();
    for line in lines {
        hashmap.insert(line[..3].to_string(), vec![line[7..10].to_string(), line[12..15].to_string()]);
    }
    let mut count: usize = 0;
    let mut next: &String = &"AAA".to_string();
    let mut vec_next: &Vec<String>;
    while next != "ZZZ" {
        //println!("The next in search is: {}", next);
        vec_next = hashmap.get(next).unwrap();
        if &directions[count%directions.len()] == &'L' {
            next = &vec_next[0]
        }
        else {
            next = &vec_next[1]
        }
        count += 1;
    }

    println!("\"ZZZ\" Found. Ammount of tries: {}", count);
}

fn part2(input:String) {
    let mut lines = input.lines();
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let collection = lines.next().unwrap();
    let directions: Vec<char> = collection.chars().collect();
    lines.next();
    for line in lines {
        hashmap.insert(line[2..3].to_string(), vec![line[7..10].to_string(), line[12..15].to_string(), line[0..3].to_string()]);
    }
    println!("{:?}", hashmap);
    let mut count: usize = 0;
    let mut next: &String = &"__A".to_string(); //Vec<String> all indexes of A
    // let mut end: Vec<String> new vec(); all ends with Z
    let mut vec_next: &Vec<String>;
    while next != "Z" {
        //println!("The next in search is: {}", next);
        vec_next = hashmap.get(next).unwrap();
        if &directions[count%directions.len()] == &'L' {
            next = &vec_next[0]
        }
        else {
            next = &vec_next[1]
        }
        count += 1;
    }
}