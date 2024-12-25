use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Result},
    path::Path,
};

pub fn parse_file() -> Result<()> {
    let path = Path::new("./src/testdata.txt");

    let file = File::open(path)?;

    let reader = BufReader::new(file);
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(ln) => rows.push(ln.chars().collect::<Vec<char>>()),
            Err(e) => eprintln!("Error parsing file"),
        }
    }
    println!("Rows: {:?}", rows);
    Ok(())
}
