use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let result = read_file();
    println!("{:?}", result);
}

fn read_file() -> io::Result<()> {
    let path = Path::new("./src/inputdata/testdata.txt");

    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut reversed_line: Vec<&str> = Vec::new();
        //println!("{:?}", ln.split("").collect())
        let mut rev_chars = String::new();
        match line {
            Ok(ln) => rev_chars = ln.chars().rev().collect(),
            Err(err) => eprintln!("Error reading line"),
        }
        // let reversed_line: Vec<&str> = line?.split("");
        println!("{:?}", rev_chars);
    }

    Ok(())
}
