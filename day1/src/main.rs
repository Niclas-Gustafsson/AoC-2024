use std::{collections::HashMap, fs, path::Path};

fn main() {
    partTwo();
}

fn parseFile() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();

    let path = Path::new("./src/input_data/day1-inputdata.txt");

    for line in fs::read_to_string(path).unwrap().lines() {
        let trimmed_line: Vec<_> = line.split("   ").collect();
        left.push(trimmed_line[0].parse::<i32>().unwrap());

        right.push(trimmed_line[1].parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    for i in 0..left.len() {
        if left[i] < right[i] {
            distances.push(right[i] - left[i]);
        } else if (right[i] < left[i]) {
            distances.push(left[i] - right[i]);
        } else {
            distances.push(left[i] - right[i]);
        }
    }
    //answer
    println!("distances: {:?}", distances.iter().sum::<i32>());
}
/* --------------------------------- PART TWO ------------------------------- */
fn partTwo() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut distances: HashMap<i32, i32> = HashMap::new();

    let path = Path::new("./src/input_data/day1-inputdata.txt");

    for line in fs::read_to_string(path).unwrap().lines() {
        let trimmed_line: Vec<_> = line.split("   ").collect();
        left.push(trimmed_line[0].parse::<i32>().unwrap());

        right.push(trimmed_line[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    for item in right {
        *distances.entry(item).or_insert(0) += 1;
    }

    let mut sum: Vec<i32> = Vec::new();

    for i in 0..left.len() {
        match distances.get(&left[i]) {
            Some(&count) => sum.push(left[i] * &count),
            _ => sum.push(0),
        };
    }
    //answer
    println!("{:?}", sum.iter().sum::<i32>());
}
