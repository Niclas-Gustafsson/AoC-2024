use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use regex::Regex;

pub fn read_file() -> io::Result<i32> {
    let path = Path::new("./src/inputdata/inputdata.txt");
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut word_count = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        matrix.push(line.chars().collect())
    }

    for (i, row) in matrix.iter().enumerate() {
        let mut word: Vec<char> = Vec::new();
        if i > 0 && i < matrix.len() - 1 {
            for (j, char) in row.iter().enumerate() {
                if j > 0 && j < row.len() - 1 && *char == 'A' {
                    match Regex::new(r"^(MAS|SAM)$") {
                        Ok(pattern) => {
                            //collect letters top-left-bottom-right
                            word.push(matrix[i - 1][j - 1]);
                            word.push(matrix[i][j]);
                            word.push(matrix[i + 1][j + 1]);

                            let mut s: String = word.iter().collect();
                            if !pattern.is_match(&s) {
                                s = String::from("");
                                word.clear();
                                continue;
                            }

                            word.clear();
                            //collect letters top-right-bottom-left
                            word.push(matrix[i - 1][j + 1]);
                            word.push(matrix[i][j]);
                            word.push(matrix[i + 1][j - 1]);
                            s = word.iter().collect();

                            if pattern.is_match(&s) {
                                word_count += 1;
                                s = String::from("");
                                word.clear();
                            }
                        }
                        Err(e) => eprintln!("Error parsing regex {}", e),
                    }
                }
            }
        }
    }
    println!("{}", word_count);
    Ok(word_count)
}
