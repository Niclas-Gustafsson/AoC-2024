use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};
/*
part two unfinished
*/
fn main() {
    // let part_one_res = part_one();

    let part_two_res = part_two();
    // println!("Part one: {:?}", part_one_res);

    println!("Part two: {:?}", part_two_res);
}

fn part_one() -> io::Result<()> {
    let path = Path::new("./src/inputdata.txt");

    let file = File::open(path)?;

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();
    let mut sum = 0;
    let reader = io::BufReader::new(file);
    let mut is_rule: bool = true;

    for line in reader.lines() {
        match line {
            Ok(content) => {
                if content.is_empty() {
                    is_rule = false;
                    continue;
                }
                if is_rule {
                    let split_line: Vec<_> = content.trim().split("|").collect();
                    rules
                        .entry(split_line[0].to_string())
                        .or_insert_with(Vec::new)
                        .push(split_line[1].to_string());
                } else {
                    let split_line: Vec<_> =
                        content.trim().split(",").map(|x| x.to_string()).collect();
                    updates.push(split_line);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    for update in updates {
        let mut is_update_correct: bool = true;

        for (index, page) in update.iter().enumerate() {
            if let Some(page_rules) = rules.get(page) {
                for rule in page_rules {
                    if let Some(rule_pos) = update.iter().position(|u| u == rule) {
                        if rule_pos <= index {
                            is_update_correct = false;
                        }
                    }
                }
            }
        }

        println!("Update: {:?} is correct: {:?}", update, is_update_correct);
        if is_update_correct {
            let middle_value: i32 = update[update.len() / 2].parse().unwrap();
            sum += middle_value;
        }
    }
    println!("Sum: {}", sum);

    Ok(())
}

fn part_two() -> io::Result<()> {
    let path = Path::new("./src/testdata.txt");

    let file = File::open(path)?;

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();
    let mut sum = 0;
    let reader = io::BufReader::new(file);
    let mut is_rule: bool = true;

    for line in reader.lines() {
        match line {
            Ok(content) => {
                if content.is_empty() {
                    is_rule = false;
                    continue;
                }
                if is_rule {
                    let split_line: Vec<_> = content.trim().split("|").collect();
                    rules
                        .entry(split_line[0].to_string())
                        .or_insert_with(Vec::new)
                        .push(split_line[1].to_string());
                } else {
                    let split_line: Vec<_> =
                        content.trim().split(",").map(|x| x.to_string()).collect();
                    updates.push(split_line);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    println!("Rules: {:?}", rules);
    println!("updates: {:?}", updates);
    let mut failed_updates: Vec<Vec<String>> = Vec::new();
    for update in updates {
        println!("UPDATE: {:?}", update);
        let mut is_update_correct: bool = true;
        let mut sorted_updates: Vec<String> = Vec::new();
        for (index, page) in update.iter().enumerate() {
            // println!("PAGE: {}", page);
            if let Some(page_rules) = rules.get(page) {
                for rule in page_rules {
                    // println!("RULE: {}", rule);
                    if let Some(rule_pos) = update.iter().position(|u| u == rule) {
                        // println!("RULE EXISTS");
                        if rule_pos < index {
                            failed_updates.push(update.clone());
                            is_update_correct = false;
                            /*   if rule_pos as i32 - 1 < 0 {
                                sorted_updates.insert(0, page.clone());
                            } else {
                                sorted_updates.insert(rule_pos - 1, page.clone());
                            } */

                            break;
                        } else if rule_pos > index {
                            break;
                        }
                    } else {
                        // println!("PAGE IS NOT A KEY IN RULE: {}", page);
                        // sorted_updates.push(page.clone());
                        break;
                    }

                    println!("sorted updates: {:?}", sorted_updates);
                    // sorted_updates.push(page.clone());
                }
            } else {
                println!("PAGE IS NOT A KEY IN RULE: {}", page);
                // sorted_updates.push(page.clone());
            }
        }
    }
    println!("FAILED UPDATES: {:?}", failed_updates);
    println!("Sum: {}", sum);

    Ok(())
}

fn part_two_2() -> io::Result<()> {
    let path = Path::new("./src/testdata.txt");

    let file = File::open(path)?;

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();
    let mut sum = 0;
    let reader = io::BufReader::new(file);
    let mut is_rule: bool = true;

    for line in reader.lines() {
        match line {
            Ok(content) => {
                if content.is_empty() {
                    is_rule = false;
                    continue;
                }
                if is_rule {
                    let split_line: Vec<_> = content.trim().split("|").collect();
                    rules
                        .entry(split_line[0].to_string())
                        .or_insert_with(Vec::new)
                        .push(split_line[1].to_string());
                } else {
                    let split_line: Vec<_> =
                        content.trim().split(",").map(|x| x.to_string()).collect();
                    updates.push(split_line);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    println!("Rules: {:?}", rules);
    println!("updates: {:?}", updates);
    let mut failed_updates: Vec<Vec<String>> = Vec::new();
    Ok(())
}

fn is_update_sorted() {}
