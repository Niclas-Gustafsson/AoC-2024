use num_format::{Locale, ToFormattedString};
use regex::Regex;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    let result = read_file();
    if let Ok(value) = result {
        let to_i64: i64 = value as i64;
        let formatted_sum = to_i64.to_formatted_string(&Locale::en);
        println!("Total resulting sum: {}", formatted_sum);
    }
}

fn read_file() -> io::Result<i32> {
    let path: &Path = Path::new("./src/input-data/input-data.txt");

    let file = File::open(path)?;

    let reader = io::BufReader::new(file);
    let mut total_sum: i32 = 0;
    for line in reader.lines() {
        match line {
            Ok(content) => total_sum += parse_line_two(content),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(total_sum)
}

fn parse_line(line: String) -> i32 {
    let mut line_total_sum: i32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();

    for cap in re.captures_iter(&line) {
        let first_number: i32 = cap[1].parse().unwrap();
        let second_number: i32 = cap[2].parse().unwrap();
        // println!("Numbers: {} and {}", first_number, second_number);
        line_total_sum += first_number * second_number;
    }

    line_total_sum
}
/* --------------------------------- PART TWO ----------------------------- */
fn parse_line_two(line: String) -> i32 {
    let mut line_total: i32 = 0;

    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();

    let mut disable_multiply: bool = false;
    for cap in re.captures_iter(&line) {
        let capture: String = cap[0].to_string();

        if capture == "do()" {
            disable_multiply = false;
        } else if capture == "don't()" {
            disable_multiply = true;
        } else if capture.starts_with("mul(") {
            if !disable_multiply {
                println!("disable_multiply is: {}", disable_multiply);
                let num1 = cap
                    .get(3)
                    .expect("No group 3, but mul(...) matched")
                    .as_str();
                let num2 = cap
                    .get(4)
                    .expect("No group 4, but mul(...) matched")
                    .as_str();
                println!("Num 1: {:?}", num1);
                println!("Num 2: {:?}", num2);

                let val1: i32 = num1.parse().unwrap();
                let val2: i32 = num2.parse().unwrap();
                line_total += val1 * val2;
            }
        }
    }
    line_total
}
