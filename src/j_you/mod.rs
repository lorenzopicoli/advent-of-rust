use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug, PartialEq)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Start,
    Ground,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up = 1,
    Down,
    Left,
    Right,
}

fn char_to_direction(c: char) -> Tile {
    match c {
        '|' => Tile::NS,
        '-' => Tile::EW,
        'L' => Tile::NE,
        'J' => Tile::NW,
        '7' => Tile::SW,
        'F' => Tile::SE,
        'S' => Tile::Start,
        '.' => Tile::Ground,
        _ => panic!("Invalid direction"),
    }
}

fn map(reader: BufReader<File>) -> (Vec<Vec<Tile>>, (usize, usize)) {
    let mut map = Vec::new();
    let mut start_pos = (0, 0);
    for line in reader.lines() {
        let mut row = Vec::new();
        for c in line.unwrap().chars() {
            if c == 'S' {
                start_pos = (map.len(), row.len());
            }
            row.push(char_to_direction(c));
        }
        map.push(row);
    }
    (map, start_pos)
}

fn next_tile(map: &[Vec<Tile>], action_done: Direction, next_pos: (usize, usize)) -> Option<&Tile> {
    if next_pos.0 >= map.len() {
        return None;
    }

    if next_pos.1 >= map[next_pos.0].len() {
        return None;
    }

    let next_tile = &map[next_pos.0][next_pos.1];
    let is_allowed = match action_done {
        Direction::Up => *next_tile == Tile::NS || *next_tile == Tile::SE || *next_tile == Tile::SW,
        Direction::Down => {
            *next_tile == Tile::NS || *next_tile == Tile::NE || *next_tile == Tile::NW
        }
        Direction::Left => {
            *next_tile == Tile::EW || *next_tile == Tile::NE || *next_tile == Tile::SE
        }
        Direction::Right => {
            *next_tile == Tile::EW || *next_tile == Tile::NW || *next_tile == Tile::SW
        }
    };

    if is_allowed {
        Some(next_tile)
    } else {
        None
    }
}

fn next_direction(current_tile: &Tile, action_done: Direction) -> Direction {
    match current_tile {
        Tile::NS => match action_done {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            Direction::Right => panic!("Invalid direction"),
            Direction::Left => panic!("Invalid direction"),
        },
        Tile::EW => match action_done {
            Direction::Up => panic!("Invalid direction"),
            Direction::Down => panic!("Invalid direction"),
            Direction::Right => Direction::Right,
            Direction::Left => Direction::Left,
        },
        Tile::NE => match action_done {
            Direction::Up => panic!("Invalid direction"),
            Direction::Down => Direction::Right,
            Direction::Right => panic!("Invalid direction"),
            Direction::Left => Direction::Up,
        },
        Tile::NW => match action_done {
            Direction::Up => panic!("Invalid direction"),
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Left => panic!("Invalid direction"),
        },
        Tile::SE => match action_done {
            Direction::Up => Direction::Right,
            Direction::Down => panic!("Invalid direction"),
            Direction::Right => panic!("Invalid direction"),
            Direction::Left => Direction::Down,
        },
        Tile::SW => match action_done {
            Direction::Up => Direction::Left,
            Direction::Down => panic!("Invalid direction"),
            Direction::Right => Direction::Down,
            Direction::Left => panic!("Invalid direction"),
        },
        Tile::Start => panic!("Invalid tile"),
        Tile::Ground => panic!("Invalid tile"),
    }
}

fn next_pos(start_pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if start_pos.0 == 0 {
                return None;
            }
            Some((start_pos.0 - 1, start_pos.1))
        }
        Direction::Down => Some((start_pos.0 + 1, start_pos.1)),
        Direction::Left => {
            if start_pos.1 == 0 {
                return None;
            }
            Some((start_pos.0, start_pos.1 - 1))
        }
        Direction::Right => Some((start_pos.0, start_pos.1 + 1)),
    }
}
// The length is just the length of the points, but I'm trying to keep the solution I had for part 1
fn find_loop(
    map: &Vec<Vec<Tile>>,
    start_pos: (usize, usize),
) -> (usize, (Vec<(usize, usize)>, Vec<(usize, usize)>)) {
    // Ways we can go from the start position
    let possible_directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let mut vertical_loop_points = Vec::new();
    let mut horizontal_loop_points = Vec::new();
    for direction in possible_directions {
        let mut current_direction = direction;
        let mut previous_direction = direction;
        let mut current_pos = start_pos;
        // The length is just the length of the points, but I'm trying to keep the solution I had for part 1
        let mut loop_length = 0;
        // loop_points = Vec::new();
        vertical_loop_points = Vec::new();
        horizontal_loop_points = Vec::new();
        loop {
            // loop_points.push(current_pos);
            let next_pos = next_pos(current_pos, current_direction);
            if next_pos.is_none() {
                break;
            }
            let next_pos = next_pos.unwrap();
            if next_pos == start_pos {
                // return (loop_length + 1, loop_points);
                let changed_direction = ((current_direction == Direction::Up
                    || current_direction == Direction::Down)
                    && (previous_direction == Direction::Left
                        || previous_direction == Direction::Right))
                    || ((previous_direction == Direction::Up
                        || previous_direction == Direction::Down)
                        && (current_direction == Direction::Left
                            || current_direction == Direction::Right));

                if changed_direction {
                    horizontal_loop_points.push(next_pos);
                    horizontal_loop_points.push(current_pos);
                    vertical_loop_points.push(next_pos);
                    vertical_loop_points.push(current_pos);
                } else if current_direction == Direction::Up || current_direction == Direction::Down
                {
                    vertical_loop_points.push(next_pos);
                    vertical_loop_points.push(current_pos);
                } else {
                    horizontal_loop_points.push(next_pos);
                    horizontal_loop_points.push(current_pos);
                }
                return (
                    loop_length + 1,
                    (vertical_loop_points, horizontal_loop_points),
                );
            }
            match next_tile(map, current_direction, next_pos) {
                Some(next_tile) => {
                    if next_tile == &Tile::Ground {
                        break;
                    }
                    let next_direction = next_direction(next_tile, current_direction);
                    loop_length += 1;
                    let changed_direction = ((current_direction == Direction::Up
                        || current_direction == Direction::Down)
                        && (previous_direction == Direction::Left
                            || previous_direction == Direction::Right))
                        || ((previous_direction == Direction::Up
                            || previous_direction == Direction::Down)
                            && (current_direction == Direction::Left
                                || current_direction == Direction::Right));

                    if changed_direction {
                        horizontal_loop_points.push(current_pos);
                        vertical_loop_points.push(current_pos);
                    } else if current_direction == Direction::Up
                        || current_direction == Direction::Down
                    {
                        vertical_loop_points.push(current_pos);
                    } else {
                        horizontal_loop_points.push(current_pos);
                    }
                    previous_direction = current_direction;
                    current_direction = next_direction;
                    current_pos = next_pos;
                }
                None => {
                    break;
                }
            };
        }
    }
    panic!("No loop found");
}

pub fn part1(reader: BufReader<File>) -> io::Result<i32> {
    let (map, start_pos) = map(reader);
    let (loop_length, _) = find_loop(&map, start_pos);
    Ok((loop_length as f64 / 2_f64).floor() as i32)
}

pub fn part2(reader: BufReader<File>) -> io::Result<i32> {
    // The length is just the length of the points, but I'm trying to keep the solution I had for part 1
    let (map, start_pos) = map(reader);
    let (_, loop_points) = find_loop(&map, start_pos);

    let mut v_results = Vec::new();

    let (vertical_loop_points, horizontal_loop_points) = loop_points;

    // println!("VErtical ranges: {:?}", vertical_ranges);
    for row in 0..map.len() {
        let mut is_in = false;
        for col in 0..map[row].len() {
            let is_crossing_vertical = vertical_loop_points
                .iter()
                .any(|x| x.0 == row && x.1 == col);

            let is_crossing_horizontal = horizontal_loop_points
                .iter()
                .any(|x| x.0 == row && x.1 == col);

            if is_crossing_vertical
                && (map[row][col] == Tile::NE
                    || map[row][col] == Tile::NW
                    || map[row][col] == Tile::NS
                    // Would break on example. What I should do is see which pipe the S would be replaced by. If it's a north
                    // facing pipe, then it should be here, if not, it should not be considered
                    || map[row][col] == Tile::Start)
            {
                is_in = !is_in;
            } else if is_crossing_horizontal {
                continue;
            } else if is_in {
                v_results.push((row, col));
            }
        }
    }
    Ok(v_results.len() as i32)
}

pub fn solution() {
    let path = Path::new("src/j_you/input.txt");
    // let path = Path::new("src/j_you/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 10, part 1 {}", part1.unwrap());
    println!("Day 10, part 2 {}", part2.unwrap());
}
