use std::{fs};
use std::collections::HashMap;
fn main() {
    // Parser reading file as string
    let input: String = fs::read_to_string("input.text").expect("Error in reading the file").to_string();
    part1(input.clone());
    part2(input.clone());
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
            next = &vec_next[0];
        }
        else {
            next = &vec_next[1];
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
    let mut vec_start:  Vec<&str> = Vec::new();
    lines.next();
    for line in lines {
        let string1 = &line[..3];
        hashmap.insert(string1.to_string(), vec![line[7..10].to_string(), line[12..15].to_string()]);
        if line.chars().nth(2).unwrap() == 'A' {
             vec_start.push(string1);
        }
    }
    let mut vec_count:  Vec<usize> = Vec::new();
    //let mut vec_wrap:  Vec<&String> = Vec::new(); supposidly dont need...
    for index in 0..vec_start.len() {
        vec_count.push(0);
        let mut next:  &str = vec_start[index];
        let mut vec_next: &Vec<String>;
        while next.chars().nth(2).unwrap() != 'Z' {
            vec_next = hashmap.get(next).unwrap();
            if &directions[vec_count[index]%directions.len()] == &'L' {
                next = &vec_next[0];
            }
            else {
                next = &vec_next[1];
            }
            vec_count[index] += 1;
        }
    }

    let mut output: usize = 0;
    output = lcm(vec_count[0], vec_count[1]);
    for index in (1..vec_count.len()) {
        output = lcm(output, vec_count[index]);
    }
    println!("\"Z\" Found. Ammount of tries: {}", output);
} 

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}