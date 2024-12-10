use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

type RowsAndCols = (Vec<Vec<char>>, Vec<Vec<char>>);
fn main() {
    let result = read_file();
    println!("{:?}", result);
}

fn read_file() -> io::Result<(i32)> {
    let path = Path::new("./src/inputdata/inputdata.txt");
    let word = "XMAS";
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut word_count = 0;
    let (rows, cols) = create_grid(&lines);

    word_count += find_word(&rows, word);

    word_count += find_word(&cols, word);
    word_count += top_left_bottom_right(&rows, word);
    word_count += top_right_bottom_left(&rows, word);
    word_count += bottom_left_top_right(&rows, word);
    word_count += bottom_right_top_left(&rows, word);
    // let sum = rows_word_count + cols_word_count;
    println!("{}", word_count);
    Ok(word_count)
}
fn create_grid(lines: &Vec<String>) -> RowsAndCols {
    let mut rows_vec: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        rows_vec.push(row.clone());
    }

    let mut cols_vec: Vec<Vec<char>> = vec![Vec::new(); rows_vec[0].len()];

    for row in &rows_vec {
        for (col_idx, &ch) in row.iter().enumerate() {
            cols_vec[col_idx].push(ch);
        }
    }

    (rows_vec, cols_vec)
}

fn find_word(characters: &Vec<Vec<char>>, word: &str) -> i32 {
    println!("{}", word.len());
    let mut word_count = 0;
    //remove one because it's going to be added by the index where char == 'X' in inner loop
    let word_offset = word.len() - 1;
    //iterate over the vector of vectors
    for vec in characters {
        for (index, char) in vec.iter().enumerate() {
            if *char == 'X' {
                if index + word_offset < vec.len() {
                    //is safe to slice vector to the right
                    let word_of_slice: String = vec[index..=index + word_offset].iter().collect();
                    if word_of_slice.as_str() == word {
                        word_count += 1;
                    }
                }
                if ((index as isize) - (word_offset as isize)) >= 0 {
                    //is safe to slice vector to the left
                    let word_of_slice: String =
                        vec[index - word_offset..=index].iter().rev().collect();
                    if word_of_slice.as_str() == word {
                        word_count += 1;
                    }
                }
            }
        }
    }
    println!("word count{}", word_count);
    word_count
}

//Look for word diagonally down-right
fn top_left_bottom_right(rows: &Vec<Vec<char>>, word: &str) -> i32 {
    //remove one because it's going to be added by the index where char == 'X' in inner loop
    let word_offset = word.len() - 1;
    let mut word_count = 0;
    println!("rows length: {:?}", rows.len());
    for (vec_index, vec) in rows.iter().enumerate() {
        println!("Vec index: {:?}", vec_index);

        for (index, char) in vec.iter().enumerate() {
            let mut search_word: Vec<char> = Vec::new();
            //traverse to the right is safe
            if *char == 'X' && index + word_offset < vec.len() && vec_index + 3 < rows.len() {
                search_word.push(*char);
                search_word.push(rows[vec_index + 1][index + 1]);

                search_word.push(rows[vec_index + 2][index + 2]);

                search_word.push(rows[vec_index + 3][index + 3]);

                if search_word.len() == word.len() {
                    let word_of_slice: String = search_word.clone().into_iter().collect();
                    if word_of_slice.as_str() == word {
                        word_count += 1;
                        search_word = Vec::new();
                    }
                }
            }
        }
    }
    println!("Word count from top-left-to-bottom-right: {}", word_count);
    word_count
}

//Look for word diagonally down-left
fn top_right_bottom_left(rows: &Vec<Vec<char>>, word: &str) -> i32 {
    //remove one because it's going to be added by the index where char == 'X' in inner loop
    let word_offset = word.len() - 1;
    let mut word_count = 0;
    for (vec_index, vec) in rows.iter().enumerate() {
        for (index, char) in vec.iter().rev().enumerate() {
            let mut search_word: Vec<char> = Vec::new();
            let col_index = vec.len() - 1 - index;

            if *char == 'X' && col_index >= word_offset && vec_index + word_offset < rows.len() {
                search_word.push(*char);
                search_word.push(rows[vec_index + 1][col_index - 1]);
                search_word.push(rows[vec_index + 2][col_index - 2]);
                search_word.push(rows[vec_index + 3][col_index - 3]);

                if search_word.len() == word.len() {
                    let word_of_slice: String = search_word.clone().into_iter().collect();
                    if word_of_slice.as_str() == word {
                        word_count += 1;
                        search_word = Vec::new();
                    }
                }
            }
        }
    }
    println!("Word count from top-right-to-bottom-left: {}", word_count);
    word_count
}
//Look for word diagonally up-right
fn bottom_left_top_right(rows: &Vec<Vec<char>>, word: &str) -> i32 {
    let word_offset = word.len() - 1;
    let mut word_count = 0;

    let reversed_rows: Vec<Vec<char>> = rows.iter().rev().cloned().collect();

    for (vec_index, vec) in reversed_rows.iter().enumerate() {
        for (index, char) in vec.iter().enumerate() {
            let mut search_word: Vec<char> = Vec::new();

            if *char == 'X'
                && index + word_offset < vec.len()
                && vec_index + word_offset < rows.len()
            {
                search_word.push(*char);
                search_word.push(reversed_rows[vec_index + 1][index + 1]);
                search_word.push(reversed_rows[vec_index + 2][index + 2]);
                search_word.push(reversed_rows[vec_index + 3][index + 3]);

                if search_word.len() == word.len() {
                    let word_of_slice: String = search_word.clone().into_iter().collect();
                    if word_of_slice.as_str() == word {
                        word_count += 1;
                        search_word = Vec::new();
                    }
                }
            }
        }
    }

    println!("Word count from bottom-left-to-top-right: {}", word_count);
    word_count
}
//Look for word diagonally up-left
fn bottom_right_top_left(rows: &Vec<Vec<char>>, word: &str) -> i32 {
    let word_offset = word.len() - 1;
    let mut word_count = 0;

    let reversed_rows: Vec<Vec<char>> = rows.iter().rev().cloned().collect();

    for (vec_index, vec) in reversed_rows.iter().enumerate() {
        for (index, char) in vec.iter().rev().enumerate() {
            let mut search_word: Vec<char> = Vec::new();

            let col_index = vec.len() - 1 - index;
            //Should match char against the first letter in the given word instead of hard coding it. #lazy
            if *char == 'X' && col_index >= word_offset && vec_index + word_offset < rows.len() {
                search_word.push(*char);
                search_word.push(reversed_rows[vec_index + 1][col_index - 1]);
                search_word.push(reversed_rows[vec_index + 2][col_index - 2]);
                search_word.push(reversed_rows[vec_index + 3][col_index - 3]);

                if search_word.len() == word.len() {
                    let word_of_slice: String = search_word.clone().into_iter().collect();
                    if word_of_slice.as_str() == word {
                        word_count += 1;
                        search_word = Vec::new();
                    }
                }
            }
        }
    }

    println!("Word count from bottom-right to top-left: {}", word_count);
    word_count
}
