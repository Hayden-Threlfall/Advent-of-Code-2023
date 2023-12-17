use std::{fs};

fn main() {
    let input: String = fs::read_to_string("input.text").expect("Error in reading the file").to_string();
    // part1(input.clone());
    part2(input.clone());
}

fn part1(input: String) {
    let lines = input.lines();
    let mut sensor: Vec<Vec<i32>> = Vec::new();
    let mut count: i32 = 0;
    for line in lines {
        sensor = vec![line.split(" ").map(|x|->i32{x.parse().unwrap()}).collect()];
        let mut vIndex = 0;
        while !is_zero(sensor[vIndex].clone()) {
            sensor.push(Vec::new());
            for index in (0..sensor.clone()[vIndex].len()-1) {
                let mut new_val: i32 = sensor.clone()[vIndex][index+1] - sensor.clone()[vIndex][index];
                sensor[vIndex+1].push(new_val);
                
            }
            vIndex += 1;

        }
        
        for slice in &sensor {
            count += slice.last().unwrap();
        }
        
    }
    println!("PART 1 :--: The final addition of the hisotry is: {}", count);
}

fn part2(input: String) {
    let lines = input.lines();
    let mut sensor: Vec<Vec<i32>> = Vec::new();
    let mut count: i32 = 0;
    for line in lines {
        sensor = vec![line.split(" ").map(|x|->i32{x.parse().unwrap()}).collect()];
        let mut vIndex = 0;
        while !is_zero(sensor[vIndex].clone()) {
            sensor.push(Vec::new());
            for index in (0..sensor.clone()[vIndex].len()-1) {
                let mut new_val: i32 = sensor.clone()[vIndex][index+1] - sensor.clone()[vIndex][index];
                sensor[vIndex+1].push(new_val);
                
            }
            vIndex += 1;

        }
        
        for slice in &sensor {
            count += slice.last().unwrap();
        }
        
    }
    println!("PART 1 :--: The final addition of the hisotry is: {}", count);
}


fn is_zero(buf: Vec<i32>) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}