use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_file() -> io::Result<()> {
    let path = Path::new("./src/inputdata.txt");

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut check_sum = 0;

    for line in reader.lines() {
        match line {
            Ok(ln) => check_sum += collect_file_id(&ln.chars().collect::<Vec<char>>()),
            Err(e) => eprintln!("{}", e),
        }
    }
    println!("sum: {}", check_sum);
    Ok(())
}
pub fn collect_file_id(line: &Vec<char>) -> u64 {
    let mut collected_ids: Vec<String> = line
        .iter()
        .enumerate()
        .flat_map(|(index, c)| {
            let count = c.to_string().parse::<u16>().unwrap();
            let id = if index % 2 != 0 {
                String::from(".")
            } else {
                // let temp = ((index / 2) as u8 + b'0') as u32;
                println!("index: {}", index);
                format!("{}", index / 2)
            };
            println!("ID from flatmap: {}", id);
            std::iter::repeat(id).take(count as usize)
        })
        .collect();
    println!("{:?}", collected_ids);
    let sorted_line = sort_line(&mut collected_ids);

    summarize(&sorted_line)
}

pub fn sort_line(ids: &mut Vec<String>) -> Vec<String> {
    let mut j = ids.len() - 1;
    for i in 0..ids.len() {
        if ids[i] == '.'.to_string() {
            while j > i && ids[j] == '.'.to_string() {
                if j == 0 {
                    break;
                }
                j -= 1;
            }

            if j > i && ids[j] != '.'.to_string() {
                ids.swap(i, j);
                j -= 1;
            }
        }
    }

    println!("ids: {:?}", ids);
    ids.to_vec()
}

pub fn summarize(ids: &Vec<String>) -> u64 {
    let res: Vec<u64> = ids
        .iter()
        .map(|item| {
            if *item != '.'.to_string() {
                return item.to_string().parse::<u64>().unwrap();
            }
            0
        })
        .collect();
    let mut sum = 0;
    for (index, num) in res.iter().enumerate() {
        sum += index as u64 * num;
    }
    println!("sum: {}", sum);
    sum as u64
}
