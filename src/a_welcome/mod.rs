use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn part1(reader: BufReader<File>) -> io::Result<u32> {
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let mut first = 0;
        let mut last = 0;

        // Find first
        for char in line.chars() {
            if let Some(v) = char.to_digit(10) {
                first = v;
                break;
            }
        }

        // Find last
        for char in line.chars().rev() {
            if let Some(v) = char.to_digit(10) {
                last = v;
                break;
            }
        }

        sum += (first * 10) + last;
    }

    Ok(sum)
}

pub fn part2(reader: BufReader<File>) -> io::Result<u32> {
    let written = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;
    let radix = 10;

    for line in reader.lines() {
        let line = line?;
        let mut first = 0;
        let mut last = 0;

        let mut running_word = "".to_string();
        // Find first
        for char in line.chars() {
            if let Some(v) = char.to_digit(radix) {
                first = v;
                break;
            } else {
                running_word.push(char);
                if let Some(pos) = written.iter().position(|r| running_word.contains(r)) {
                    first = (pos as u32) + 1;
                    break;
                }
            }
        }

        running_word = "".to_string();

        // Find last
        // Iterate through the reversed string, break after we find the first digit
        for char in line.chars().rev() {
            if let Some(v) = char.to_digit(radix) {
                last = v;
                break;
            } else {
                // insert at the beginning of the string, because we're reading right to left
                running_word.insert(0, char);
                if let Some(pos) = written.iter().position(|r| running_word.contains(r)) {
                    last = (pos as u32) + 1;
                    break;
                }
            }
        }

        sum += (first * 10) + last;
    }

    Ok(sum)
}

pub fn solution() -> io::Result<u32> {
    let path = Path::new("src/a_welcome/input.txt");
    // let path = Path::new("src/a_welcome/example.txt");
    // let path = Path::new("src/a_welcome/example2.txt");
    let mut file = File::open(&path)?;
    let part1 = part1(io::BufReader::new(file));
    file = File::open(&path)?;
    let part2 = part2(io::BufReader::new(file));

    println!("Problem 1, part 1 {:#?}", part1);

    part2
}
