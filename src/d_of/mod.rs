use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Small helper to easily parse a string slice and return the parts in a vector
fn split_str(value: &str, sub: &str) -> Vec<String> {
    let split = value.split(sub);
    split.map(|e| e.to_string()).collect::<Vec<String>>()
}

pub fn part1(reader: BufReader<File>) -> io::Result<i64> {
    let mut nums: [bool; 100] = [false; 100];
    let mut sum: i64 = 0;
    for line in reader.lines() {
        let mut game_score = 0;
        let split = split_str(&line?, ":");

        let solution_and_game = split_str(&split[1], "|");

        let solution = &solution_and_game[0];
        let game = &solution_and_game[1];

        let solution_parts = split_str(solution, " ");
        let game_parts = split_str(game, " ");

        for solution in solution_parts {
            if let Ok(solution) = solution.parse::<usize>() {
                nums[solution] = true;
            }
        }

        for game in game_parts {
            if let Ok(game) = game.parse::<usize>() {
                if nums[game] {
                    if game_score == 0 {
                        game_score = 1;
                    } else {
                        game_score *= 2;
                    }
                }
            }
        }

        sum += game_score;

        // reset array
        nums = [false; 100];
    }

    Ok(sum)
}

pub fn part2(reader: BufReader<File>) -> io::Result<u32> {
    let mut scratchpad_copies: [u32; 200] = [0; 200];

    for (game_id, line) in reader.lines().enumerate() {
        let mut nums: [bool; 100] = [false; 100];
        let mut game_score = 0;
        scratchpad_copies[game_id] += 1;
        let copies_of_current = scratchpad_copies[game_id];

        let split = split_str(&line?, ":");

        let solution_and_game = split_str(&split[1], "|");

        let solution = &solution_and_game[0];
        let game = &solution_and_game[1];

        let solution_parts = split_str(solution, " ");
        let game_parts = split_str(game, " ");

        for solution in solution_parts {
            if let Ok(solution) = solution.parse::<usize>() {
                nums[solution] = true;
            }
        }

        for game in game_parts {
            if let Ok(game) = game.parse::<usize>() {
                if nums[game] {
                    game_score += 1;
                    scratchpad_copies[game_id + game_score] += copies_of_current;
                }
            }
        }
    }

    Ok(scratchpad_copies.iter().sum::<u32>())
}

pub fn solution() {
    let path = Path::new("src/d_of/input.txt");
    // let path = Path::new("src/d_of/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 4, part 1 {}", part1.unwrap());
    println!("Day 4, part 2 {}", part2.unwrap());
}
