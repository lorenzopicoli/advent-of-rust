use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let delta = (b * b) - (4_f64 * a * c);
    let x1 = (-b + (delta).sqrt()) / (2_f64 * a);
    let x2 = (-b - (delta).sqrt()) / (2_f64 * a);
    (x1, x2)
}

fn split_str(value: &str, sub: &str) -> Vec<String> {
    let split = value.split(sub);
    split.map(|e| e.to_string()).collect::<Vec<String>>()
}

pub fn part1(reader: BufReader<File>) -> io::Result<i32> {
    let mut time_records: Vec<u16> = Vec::new();
    let mut distances: Vec<u16> = Vec::new();

    for (line_idx, line) in reader.lines().enumerate() {
        let mut line = line.unwrap();
        line = line.strip_prefix("Time: ").unwrap_or(&line).to_string();
        line = line.strip_prefix("Distance: ").unwrap_or(&line).to_string();
        line = line.trim().to_string();

        let values = split_str(&line, " ");

        for value in values {
            if value.is_empty() {
                continue;
            }
            match line_idx {
                0 => {
                    time_records.push(value.parse::<u16>().unwrap());
                }
                1 => {
                    distances.push(value.parse::<u16>().unwrap());
                }
                _ => {}
            }
        }
    }

    let mut result = 1;
    // Best way is to solve it as a quadratic equation. We have the distance and the time, we need to calculate
    // the speed at which we can travel to the destination, but the twist is that the time decreases every time
    // the speed increases. So the equation goes from d = t * s to d = s * (t - s) which turns into: s^2 - t * s + d = 0
    // So a = 1, b = -t, c = d
    for (i, time) in time_records.iter().enumerate() {
        let distance = distances[i];
        // Add 0.1 to the distance because we want to go farther
        let speed_bounds = quadratic(1_f64, -(*time as f64), (distance as f64) + 0.1_f64);
        let time_bounds = (
            time - speed_bounds.0.abs().ceil() as u16,
            time - speed_bounds.1.abs().ceil() as u16,
        );
        let ways_to_win = time_bounds.1 - time_bounds.0;
        result *= ways_to_win as i32;
    }

    Ok(result)
}

pub fn part2(reader: BufReader<File>) -> io::Result<u64> {
    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    for (line_idx, line) in reader.lines().enumerate() {
        let mut line = line.unwrap();
        line = line.strip_prefix("Time: ").unwrap_or(&line).to_string();
        line = line.strip_prefix("Distance: ").unwrap_or(&line).to_string();
        line = line.trim().to_string();

        let value = line.replace(' ', "");

        match line_idx {
            0 => {
                time = value.parse::<u64>().unwrap();
            }
            1 => {
                distance = value.parse::<u64>().unwrap();
            }
            _ => {}
        }
    }

    let mut result = 1;
    // Best way is to solve it as a quadratic equation. We have the distance and the time, we need to calculate
    // the speed at which we can travel to the destination, but the twist is that the time decreases every time
    // the speed increases. So the equation goes from d = t * s to d = s * (t - s) which turns into: s^2 - t * s + d = 0
    // So a = 1, b = -t, c = d
    // Add 0.1 to the distance because we want to go farther
    let speed_bounds = quadratic(1_f64, -(time as f64), (distance as f64) + 0.1_f64);
    let time_bounds = (
        time - speed_bounds.0.abs().ceil() as u64,
        time - speed_bounds.1.abs().ceil() as u64,
    );
    let ways_to_win = time_bounds.1 - time_bounds.0;
    result *= ways_to_win as u64;

    Ok(result)
}

pub fn solution() {
    let path = Path::new("src/f_/input.txt");
    // let path = Path::new("src/f_/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 6, part 1 {}", part1.unwrap());
    println!("Day 6, part 2 {}", part2.unwrap());
}
