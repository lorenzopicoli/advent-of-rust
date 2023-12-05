use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
    path::Path,
};

fn transform(input: &u64, map: (&Range<u64>, &Range<u64>)) -> u64 {
    let (source_range, dest_range) = map;
    if source_range.contains(input) {
        let offset = input - source_range.start;
        return dest_range.start + offset;
    }

    *input
}

fn split_str(value: &str, sub: &str) -> Vec<String> {
    let split = value.split(sub);
    split.map(|e| e.to_string()).collect::<Vec<String>>()
}

pub fn part1(reader: BufReader<File>) -> io::Result<u64> {
    let mut seeds: Vec<u64> = Vec::new();
    let mut transformed_indexes: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("seeds:") {
            seeds = Vec::new();
            let split = split_str(line.strip_prefix("seeds: ").unwrap(), " ");
            for s in split {
                seeds.push(s.parse().unwrap())
            }
            continue;
        }
        if line.contains(':') {
            transformed_indexes = Vec::new();
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let split = split_str(&line, " ");
        let dest = split[0].parse::<u64>().unwrap();
        let source = split[1].parse::<u64>().unwrap();
        let length = split[2].parse::<u64>().unwrap();

        let source_range = source..(source + length + 1);
        let dest_range = dest..(dest + length + 1);

        for (i, seed) in seeds.iter_mut().enumerate() {
            if transformed_indexes.contains(&i) {
                continue;
            }
            let previous_seed = *seed;
            *seed = transform(&(*seed), (&source_range, &dest_range));
            if previous_seed != *seed {
                transformed_indexes.push(i);
            }
        }
    }
    Ok(seeds.into_iter().min().unwrap())
}

#[derive(Debug)]
struct TransformationRange {
    range: Range<u64>,
    t_id: i32,
}

fn transform_ranges(
    seed: TransformationRange,
    source: TransformationRange,
    t: i64,
    t_id: i32,
) -> Vec<TransformationRange> {
    // No intersection
    if seed.range.start >= source.range.end || seed.range.end <= source.range.start {
        return vec![seed];
    }

    let mut points = [
        seed.range.start,
        seed.range.end - 1,
        source.range.start,
        source.range.end - 1,
    ];

    points.sort();

    // First create it as an inclusive range just because it's easier for me to understand and debug
    let first = points[0]..=((points[1] as i64) - 1).max(0) as u64;
    let last = points[2] + 1..=points[3];
    let intersection =
        ((points[1] as i64 + t).max(0) as u64)..=((points[2] as i64 + t).max(0) as u64);

    // Convert to exclusive range
    // Could lead to overflow https://stackoverflow.com/questions/64393457/how-do-i-convert-a-rangeinclusive-to-a-range
    let mut first = *first.start()..(first.end() + 1);
    let mut last = *last.start()..(last.end() + 1);
    let intersection = *intersection.start()..(intersection.end() + 1);

    // Bound results by seed
    if first.start <= seed.range.start {
        first.start = seed.range.start;
    }
    if last.end >= seed.range.end {
        last.end = seed.range.end;
    }

    // The intersection is in the result for sure
    let mut result = vec![TransformationRange {
        range: intersection,
        t_id,
    }];

    if first.end - first.start > 1 {
        result.push(TransformationRange {
            range: first,
            t_id: seed.t_id,
        });
    }

    if last.end - last.start > 1 {
        result.push(TransformationRange {
            range: last,
            t_id: seed.t_id,
        });
    }

    result
}

pub fn part2(reader: BufReader<File>) -> io::Result<u64> {
    let mut seeds: Vec<TransformationRange> = Vec::new();
    let mut map_idx = 0;
    for line in reader.lines() {
        let line = line?;
        if line.contains("seeds:") {
            seeds = Vec::new();
            let split = split_str(line.strip_prefix("seeds: ").unwrap(), " ");
            for step in split.iter().step_by(2).enumerate() {
                let start = split[step.0 * 2].parse::<u64>().unwrap();
                let length = split[step.0 * 2 + 1].parse::<u64>().unwrap();
                seeds.push(TransformationRange {
                    range: start..start + length + 1,
                    t_id: -1,
                });
            }
            continue;
        }
        if line.contains(':') {
            map_idx += 1;
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let split = split_str(&line, " ");
        let dest = split[0].parse::<u64>().unwrap();
        let source = split[1].parse::<u64>().unwrap();
        let length = split[2].parse::<u64>().unwrap();

        let source_range = source..(source + length + 1);
        let dest_range = dest..(dest + length + 1);

        let mut new_seeds: Vec<TransformationRange> = Vec::new();
        for seed in &seeds {
            if seed.t_id == map_idx {
                new_seeds.push(TransformationRange {
                    range: seed.range.clone(),
                    t_id: seed.t_id,
                });
                continue;
            }
            let mut transformation = transform_ranges(
                TransformationRange {
                    range: seed.range.clone(),
                    t_id: seed.t_id,
                },
                TransformationRange {
                    range: source_range.clone(),
                    t_id: -1,
                },
                (dest_range.start as i64) - (source_range.start as i64),
                map_idx,
            );
            new_seeds.append(&mut transformation);
        }

        seeds = new_seeds;
    }

    Ok(seeds
        .iter()
        .min_by_key(|e| e.range.start)
        .unwrap()
        .range
        .start)
}

pub fn solution() {
    let path = Path::new("src/e_rust/input.txt");
    // let path = Path::new("src/e_rust/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 5, part 1 {}", part1.unwrap());
    println!("Day 5, part 2 {}", part2.unwrap());
}
