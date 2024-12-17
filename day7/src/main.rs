use core::num;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};
/*
part one unfinished
part two unfinished

*/
fn main() {
    let _ = parse_file();
}

fn parse_file() -> io::Result<()> {
    let path = Path::new("./src/inputdata.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut total_sum = 0;

    for line in reader.lines() {
        let mut target_sum = 0;
        let mut numbers: Vec<i64> = Vec::new();

        match line {
            Ok(row) => {
                let row_sum: Vec<&str> = row.split(':').collect();
                numbers = row_sum[1]
                    .trim()
                    .split(" ")
                    .map(|item| item.parse::<i64>().expect("Failed to parse integer"))
                    .collect();

                match row_sum[0].parse::<i64>() {
                    Ok(val) => target_sum = val,
                    Err(e) => eprintln!("Error parsing to integer: {}", e),
                }
            }
            Err(_) => eprintln!("Error reading line"),
        }

        total_sum += calc_sum(&numbers, target_sum);
    }

    //answer
    println!("Equation: {:?}", total_sum);
    Ok(())
}

fn calc_sum(num_vec: &Vec<i64>, target_sum: i64) -> i64 {
    let operator_slots = num_vec.len() - 1;
    let operator_combinations = 1 << operator_slots;

    for mask in 0..operator_combinations {
        let mut current_value: i64 = num_vec[0];

        for i in 0..operator_slots {
            let operator = if (mask & (1 << i)) == 0 { '+' } else { '*' };
            let next_number = num_vec[i + 1];
            // println!("{}", operator);

            match operator {
                '+' => {
                    current_value += next_number;
                }
                '*' => {
                    current_value *= next_number;
                }
                _ => {}
            }
        }

        if current_value == target_sum {
            return current_value;
        }
    }
    0
}
