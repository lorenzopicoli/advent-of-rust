use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn get_world(reader: BufReader<File>) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut world = Vec::new();
    let mut galaxy_positions = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        world.push(Vec::new());
        for ch in line.chars() {
            let length = world.len();
            world[length - 1].push(ch);
            if (ch == '#') {
                galaxy_positions.push((length - 1, world[length - 1].len() - 1));
            }
        }
    }
    (world, galaxy_positions)
}

fn get_sum(reader: BufReader<File>, expansion_rate: i32) -> io::Result<u128> {
    let (world, galaxy_positions) = get_world(reader);
    let empty_rows = world
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|ch| *ch == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let empty_cols = world[0]
        .iter()
        .enumerate()
        .filter(|(col, _)| world.iter().all(|row| row[*col] == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let mut galaxy_combinations = Vec::new();
    for (i, galaxy) in galaxy_positions.iter().enumerate() {
        for other in galaxy_positions.iter().skip(i + 1) {
            galaxy_combinations.push((galaxy, other));
        }
    }

    let distances = galaxy_combinations
        .iter()
        .map(|(g1, g2)| {
            let start_row = g1.0.min(g2.0);
            let end_row = g1.0.max(g2.0);
            let start_col = g1.1.min(g2.1);
            let end_col = g1.1.max(g2.1);
            let empty_row_between: u128 = empty_rows
                .iter()
                .filter(|row| *row > &start_row && *row < &end_row)
                .count() as u128
                * (expansion_rate as u128 - 1);
            let empty_cols_between = empty_cols
                .iter()
                .filter(|col| *col > &start_col && *col < &end_col)
                .count() as u128
                * (expansion_rate as u128 - 1);
            (
                *g1,
                *g2,
                (g1.0 as i128 - g2.0 as i128).unsigned_abs()
                    + (g1.1 as i128 - g2.1 as i128).unsigned_abs()
                    + empty_cols_between
                    + empty_row_between,
            )
        })
        .collect::<Vec<(&(usize, usize), &(usize, usize), u128)>>();
    Ok(distances.iter().map(|x| x.2).sum())
}

pub fn part1(reader: BufReader<File>) -> io::Result<u128> {
    get_sum(reader, 2)
}

pub fn part2(reader: BufReader<File>) -> io::Result<u128> {
    get_sum(reader, 1000000)
}

pub fn solution() {
    let path = Path::new("src/k_a/input.txt");
    // let path = Path::new("src/k_a/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    println!("Day 11, part 1 {}", part1.unwrap());
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 11, part 2 {}", part2.unwrap());
}
