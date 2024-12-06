use std::{fs, path::Path};

fn main() {
    parse_input();
}

fn parse_input() {
    let mut reports_safe: i32 = 0;
    let path = Path::new("./src/input_data/input_data.txt");

    //Prepare closure to parse each number to integer.
    let parse_to_int = |x: &str| -> i32 { x.parse::<i32>().unwrap_or(0) };

    //Read lines in file and parse line content to i32 vector
    for line in fs::read_to_string(path).unwrap().lines() {
        let report: Vec<i32> = line.split(" ").map(parse_to_int).collect();
        let mut safe = is_report_safe(&report);

        if !safe {
            // Try removing each element to see if the sequence becomes safe
            for i in 0..report.len() {
                let mut modified_report = report.clone();
                modified_report.remove(i);
                if is_report_safe(&modified_report) {
                    safe = true;
                    break;
                }
            }
        }

        if safe {
            reports_safe += 1;
        }
    }

    println!("Total reports safe: {:?}", reports_safe);
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        //if a report is < 2 it is safe
        return true;
    }

    let is_increasing = report[1] > report[0];
    let is_decreasing = report[1] < report[0];

    // If the report is neither increasing nor decreasing
    if !is_increasing && !is_decreasing {
        return false;
    }

    for i in 0..report.len() - 1 {
        let diff = (report[i + 1] - report[i]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if is_increasing && report[i + 1] <= report[i] {
            return false;
        }
        if is_decreasing && report[i + 1] >= report[i] {
            return false;
        }
    }
    true
}
