use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Holds the game id and each subset (times the elf showed cubes)
struct ParsedLine {
    game_id: i32,
    // Subsets are each time the elf has shown cubes
    subsets: Vec<String>,
}

/// Small helper to easily parse a string slice and return the parts in a vector
fn split_str(value: &str, sub: &str) -> Vec<String> {
    let split = value.split(sub);
    split.map(|e| e.to_string()).collect::<Vec<String>>()
}

/// Given a line line: 3 blue, 4 red; 1 red, 2 green, 6 blue
/// return the (r, g, b) values
fn count_colors(colors: Vec<String>) -> (i32, i32, i32) {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;
    for color in colors {
        let parts = split_str(&color.trim(), " ");
        let name = parts[1].clone();
        let count = parts[0].clone();
        match name.as_str() {
            "blue" => blue += count.to_string().parse::<i32>().unwrap(),
            "red" => red += count.to_string().parse::<i32>().unwrap(),
            "green" => green += count.to_string().parse::<i32>().unwrap(),
            _ => panic!("Failed to match color name"),
        }
    }

    (red, green, blue)
}

fn parse_line(line: &str) -> ParsedLine {
    let split = split_str(&line, ":");
    let game = split_str(&split[0], " ");
    let game_id = game[1].clone().parse::<i32>().unwrap();
    let subsets = split_str(&split[1], ";");

    return ParsedLine { game_id, subsets };
}

pub fn part1(reader: BufReader<File>) -> io::Result<i32> {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let mut game_ids_sum = 0;
    for line in reader.lines() {
        let parsed = parse_line(&line?);
        let mut is_game_valid = true;
        for subset in parsed.subsets {
            let colors = split_str(&subset, ",");
            let (red, green, blue) = count_colors(colors);
            is_game_valid = red <= MAX_RED && blue <= MAX_BLUE && green <= MAX_GREEN;
            if !is_game_valid {
                break;
            }
        }

        if is_game_valid {
            game_ids_sum += parsed.game_id;
        }
    }
    Ok(game_ids_sum)
}

pub fn part2(reader: BufReader<File>) -> io::Result<i32> {
    let mut total_power = 0;
    for line in reader.lines() {
        let parsed = parse_line(&line?);
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for subset in parsed.subsets {
            let colors = split_str(&subset, ",");
            let (red, green, blue) = count_colors(colors);

            if red > max_red {
                max_red = red;
            }
            if blue > max_blue {
                max_blue = blue;
            }
            if green > max_green {
                max_green = green;
            }
        }
        total_power += max_red * max_green * max_blue;
    }
    Ok(total_power)
}

pub fn solution() -> io::Result<i32> {
    let path = Path::new("src/b_to/input.txt");
    // let path = Path::new("src/b_to/example.txt");
    let mut file = File::open(&path)?;
    let part1 = part1(io::BufReader::new(file));
    file = File::open(&path)?;
    let part2 = part2(io::BufReader::new(file));

    println!("Problem 2, part 1 {:#?}", part1);

    part2
}
