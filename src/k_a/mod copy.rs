use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn get_expanded_world(reader: BufReader<File>, expansion_rate: i32) -> Vec<Vec<char>> {
    let mut world = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        world.push(Vec::new());
        let mut is_empty = true;
        for ch in line.chars() {
            if ch != '.' {
                is_empty = false;
            }
            let length = world.len();
            world[length - 1].push(ch);
        }

        if is_empty {
            let empty_row = world[world.len() - 1].clone();
            // Add extra row if whole row is empty
            for _ in 0..expansion_rate - 1 {
                world.push(empty_row.clone());
            }
        }
    }
    let mut expanded_world = world.clone();
    let mut insert_count = 0;
    for col in 0..world[0].len() {
        let is_column_empty = world.iter().all(|row| row[col] == '.');

        if is_column_empty {
            for row in 0..world.len() {
                for _ in 0..expansion_rate - 1 {
                    expanded_world[row].insert(col + insert_count, '.');
                }
            }
            insert_count += expansion_rate as usize - 1;
        }
    }

    // println!("World:");
    // for row in &expanded_world {
    //     println!("{:?}", row);
    // }
    expanded_world
}

fn get_sum(reader: BufReader<File>, expansion_rate: i32) -> io::Result<i32> {
    let world = get_expanded_world(reader, expansion_rate);
    let galaxy_positions = world
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, ch)| **ch == '#')
                .map(move |(y, _)| (x, y))
        })
        .collect::<Vec<(usize, usize)>>();

    let mut galaxy_combinations = Vec::new();
    for (i, galaxy) in galaxy_positions.iter().enumerate() {
        for other in galaxy_positions.iter().skip(i + 1) {
            galaxy_combinations.push((galaxy, other));
        }
    }

    let distances = galaxy_combinations
        .iter()
        .map(|(g1, g2)| {
            (
                *g1,
                *g2,
                (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs(),
            )
        })
        .collect::<Vec<(&(usize, usize), &(usize, usize), i32)>>();
    Ok(distances.iter().map(|x| x.2).sum())
}

pub fn part1(reader: BufReader<File>) -> io::Result<i32> {
    get_sum(reader, 2)
}

pub fn part2(reader: BufReader<File>) -> io::Result<i32> {
    get_sum(reader, 10)
}

pub fn solution() {
    // let path = Path::new("src/k_a/input.txt");
    let path = Path::new("src/k_a/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    println!("Day 11, part 1 {}", part1.unwrap());
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 11, part 2 {}", part2.unwrap());
}
