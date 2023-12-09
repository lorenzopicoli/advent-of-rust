use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn part1(reader: BufReader<File>) -> io::Result<i32> {
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line.split(' ').map(|i| i.parse::<i32>().unwrap()).collect();

        let mut all_zeros = false;
        let mut deltas = Vec::new();
        let mut current_deltas = numbers;

        deltas.push(current_deltas.clone());
        while !all_zeros {
            current_deltas = current_deltas
                .windows(2)
                .map(|i| i[1] - i[0])
                .collect::<Vec<i32>>();
            // Guess I would like to avoid this iteration, but it's so much easier
            all_zeros = current_deltas.iter().all(|&i| i == 0);
            // Should also aim to avoid this clone
            deltas.push(current_deltas.clone());
        }

        let len = deltas.len();
        deltas[len - 1].push(0);
        deltas.reverse();
        for i in 0..deltas.len() - 1 {
            let first = &deltas[i];
            let second = &deltas[i + 1];
            let extrapolation = second.last().unwrap() + first.last().unwrap();
            deltas[i + 1].push(extrapolation);
        }

        sum += deltas.last().unwrap().last().unwrap();
    }
    Ok(sum)
}

pub fn part2(reader: BufReader<File>) -> io::Result<i32> {
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line.split(' ').map(|i| i.parse::<i32>().unwrap()).collect();

        let mut all_zeros = false;
        let mut deltas = Vec::new();
        let mut current_deltas = numbers;

        deltas.push(current_deltas.clone());
        while !all_zeros {
            current_deltas = current_deltas
                .windows(2)
                .map(|i| i[1] - i[0])
                .collect::<Vec<i32>>();
            // Guess I would like to avoid this iteration, but it's so much easier
            all_zeros = current_deltas.iter().all(|&i| i == 0);
            // Should also aim to avoid this clone
            deltas.push(current_deltas.clone());
        }

        let len = deltas.len();
        deltas[len - 1].push(0);
        deltas.reverse();
        for i in 0..deltas.len() - 1 {
            let first = &deltas[i];
            let second = &deltas[i + 1];
            let extrapolation = second.first().unwrap() - first.first().unwrap();
            deltas[i + 1].insert(0, extrapolation);
        }

        sum += deltas.last().unwrap().first().unwrap();
    }
    Ok(sum)
}

pub fn solution() {
    let path = Path::new("src/i_wish/input.txt");
    // let path = Path::new("src/i_wish/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 9, part 1 {}", part1.unwrap());
    println!("Day 9, part 2 {}", part2.unwrap());
}
