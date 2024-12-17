use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
/*
part two unfinished
*/
fn main() {
    let _ = parse_file();
}

fn parse_file() -> io::Result<()> {
    let path = Path::new("./src/inputdata.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut rows: Vec<Vec<String>> = Vec::new();
    let re = Regex::new(r"\^").unwrap();
    let mut start_pos: (i32, i32) = (0, 0);
    let mut direction: String = String::from("up");

    for (index, line) in reader.lines().into_iter().enumerate() {
        let mut row: Vec<String> = Vec::new();
        match line {
            Ok(ln) => {
                let split_line: Vec<String> = ln.chars().map(|c| c.to_string()).collect();
                let start_x: Vec<_> = split_line
                    .iter()
                    .enumerate()
                    .filter_map(|(i, s)| if re.is_match(s) { Some(i as i32) } else { None })
                    .collect();

                if !start_x.is_empty() {
                    println!("Start_x: {:?}", start_x);

                    println!("line index: {:?}", index);
                    start_pos = (start_x[0], index as i32);
                }
                rows.push(split_line);
            }
            Err(_) => eprintln!("Error parsing line"),
        }
    }
    let mut walked_positions: Vec<(i32, i32)> = Vec::new();
    walked_positions.push(start_pos);

    let mut current_pos: (i32, i32) = start_pos;

    while current_pos.0 >= 0
        && current_pos.0 < rows[0].len() as i32
        && current_pos.1 >= 0
        && current_pos.1 < rows.len() as i32
    {
        if current_pos.0 + 1 == rows[(current_pos.1) as usize].len() as i32
            || current_pos.0 - 1 < 0 as i32
            || current_pos.1 + 1 == rows.len() as i32
            || current_pos.1 - 1 < 0 as i32
        {
            break;
        }

        if direction == "up" {
            if rows[(current_pos.1 - 1) as usize][(current_pos.0) as usize] != "#" {
                //walk upwards
                current_pos.1 -= 1;
                walked_positions.push(current_pos);
            } else {
                direction = String::from("right");
            }
        }

        if direction == "right" {
            if rows[(current_pos.1) as usize][(current_pos.0 + 1) as usize] != String::from("#") {
                //walk right
                current_pos.0 += 1;
                walked_positions.push(current_pos);
            } else {
                direction = String::from("down");
            }
        }
        if direction == "down" {
            if rows[(current_pos.1 + 1) as usize][(current_pos.0) as usize] != "#" {
                //walk down
                current_pos.1 += 1;
                walked_positions.push(current_pos);
            } else {
                direction = String::from("left");

                println!("{}", direction);
            }
        }

        if direction == "left" {
            if rows[(current_pos.1) as usize][(current_pos.0 - 1) as usize] != "#" {
                current_pos.0 -= 1;
                walked_positions.push(current_pos);
            } else {
                direction = String::from("up");
            }
        }
    }
    walked_positions.sort();
    walked_positions.dedup();
    println!("walked positions: {:?}", walked_positions.len());

    Ok(())
}
