use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn part1(reader: BufReader<File>) -> io::Result<i32> {
    for line in reader.lines() {}
    Ok(1)
}

pub fn part2(reader: BufReader<File>) -> io::Result<i32> {
    for line in reader.lines() {}
    Ok(1)
}

pub fn solution() -> io::Result<i32> {
    let path = Path::new("src/d_of/input.txt");
    // let path = Path::new("src/d_of/example.txt");
    let mut file = File::open(&path)?;
    let part1 = part1(io::BufReader::new(file));
    file = File::open(&path)?;
    let part2 = part2(io::BufReader::new(file));

    println!("Problem 4, part 1 {:#?}", part1);

    part2
}
