use core::num;
use std::{
    char,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn get_islands(line: String) -> (Vec<Vec<char>>, Vec<u32>) {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let chars = parts[0].chars();
    let digits = parts[1].split(',').collect::<Vec<&str>>();
    let records = digits
        .iter()
        .map(|c| {
            c.chars()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
                .to_digit(10)
                .unwrap()
        })
        .collect::<Vec<u32>>();
    let mut islands: Vec<Vec<char>> = Vec::new();

    for char in chars.clone() {
        if char == '.' {
            if !islands.last().unwrap_or(&Vec::new()).is_empty() {
                islands.push(Vec::new());
            }
        } else {
            if islands.is_empty() {
                islands.push(Vec::new());
            }
            let last_island: &mut Vec<char> = islands.last_mut().unwrap();
            last_island.push(char);
        }
    }
    islands.retain(|i| !i.is_empty());
    (islands, records)
}

fn get_islands_possibilities(
    islands: &[Vec<char>],
    records: &[u32],
    history: &[u32],
) -> (i32, Vec<u32>) {
    let mut total_combinations = 1;
    let mut island_changed_count = false;
    let mut next_history = Vec::new();
    for (i, (island, record)) in islands.iter().zip(records).enumerate() {
        if island.len() < *record as usize {
            // println!("Invalid because {} < {}", island.len(), *record);
            return (0, Vec::new());
        }
        // let number_of_consecutive_spaces = island
        //     .iter()
        //     .copied()
        //     .collect::<String>()
        //     .split('#')
        //     .map(|x| x.len())
        //     .max()
        //     .unwrap();
        // println!(
        //     "Number of consecutive spaces: {}",
        //     number_of_consecutive_spaces
        // );
        let number_of_consecutive_spaces = island.iter().filter(|x| **x == '?').count();
        if number_of_consecutive_spaces == 0 {
            next_history.push(number_of_consecutive_spaces as u32);
            continue;
        }
        let history_for_record = if island_changed_count {
            0
        } else {
            *history.get(i).unwrap_or(&0)
        };
        println!("History: {:?}", history);
        println!(
            "Number of consecutive spaces: {}",
            number_of_consecutive_spaces
        );
        println!("Island: {:?} of islands: {:?}", island, islands);
        println!("Record: {:?}", record);
        if history_for_record > number_of_consecutive_spaces as u32 {
            return (0, Vec::new());
        }
        let number_of_spaces =
            (number_of_consecutive_spaces - history_for_record as usize).max(*record as usize);
        let number_of_hashtags = island.iter().filter(|x| **x == '#').count();
        if (number_of_hashtags == record.clone() as usize) {
            return (0, Vec::new());
        }
        // println!("Number of spaces: {}", number_of_spaces);
        // println!("History for record: {}", history_for_record);
        // println!(
        // "Number of consecutive spaces: {}",
        // number_of_consecutive_spaces
        // );
        let combinations = (number_of_spaces as f32 / *record as f32).ceil() as i32;
        if history_for_record != number_of_consecutive_spaces as u32 && !island_changed_count {
            island_changed_count = true;
        }
        next_history.push(number_of_consecutive_spaces as u32);
        total_combinations *= combinations;
    }
    // println!("Total Combinations: {}", total_combinations);
    (total_combinations, next_history)
}

fn get_variations(
    curr_islands: Vec<Vec<char>>,
    qm_pos: Vec<usize>,
    island_count_goal: usize,
    call_n: i32,
) -> Vec<Vec<Vec<char>>> {
    if island_count_goal == curr_islands.len() {
        println!("Found a possibility: {:?}", curr_islands);
        return [curr_islands].to_vec();
    }

    if qm_pos.is_empty() {
        println!("Reached empty qm_pos");
        return Vec::new();
    }

    let mut sub_variations = Vec::new();
    for (i, pos) in qm_pos.iter().enumerate() {
        println!("Doing iteration {} with depth {}", i, call_n);
        let mut seen = 0;
        let mut executed_split = false;
        let mut new_islands = Vec::new();
        for island in &curr_islands {
            if executed_split {
                new_islands.push(island.clone());
                continue;
            }
            if seen + island.len() > *pos {
                let index_to_remove = pos - seen;
                let is_first_or_last = index_to_remove == 0 || index_to_remove == island.len() - 1;

                if index_to_remove == 0 {
                    new_islands.push(island[1..].to_vec());
                } else if index_to_remove == island.len() - 1 {
                    new_islands.push(island[0..island.len() - 1].to_vec())
                } else {
                    new_islands.push(island[0..index_to_remove].to_vec());
                    new_islands.push(island[index_to_remove + 1..island.len()].to_vec());
                }

                executed_split = true;
            } else {
                new_islands.push(island.clone());
            }

            seen += island.len();
        }
        let next_qm_positions = qm_pos
            .iter()
            .skip(i + 1)
            .map(|x| (*x as i32 - 1).max(0) as usize)
            .collect::<Vec<usize>>();
        println!(
            "Calling next depth with {:?}, from depth {}",
            next_qm_positions, call_n
        );
        sub_variations.append(&mut get_variations(
            new_islands.clone(),
            next_qm_positions,
            island_count_goal,
            call_n + 1,
        ));
    }
    return sub_variations;
}

pub fn part1(reader: BufReader<File>) -> io::Result<i32> {
    let mut final_combinations = 0;
    for line in reader.lines() {
        println!("-------------------------------------");
        let line = line.unwrap();
        let (islands, records) = get_islands(line.clone());
        println!("Islands:");
        for island in islands.clone() {
            println!("{:?}", island);
        }

        let island_diff = islands.len() as i32 - records.len() as i32;
        let mut total_combinations = 0;
        if island_diff == 0 {
            let (combinations, _) = get_islands_possibilities(&islands, &records, &[]);
            total_combinations = combinations;
        } else if island_diff < 1 {
            let islands_to_create = island_diff.abs();
            let points_to_add = islands_to_create;
            println!("Have to add islands {}", points_to_add);
            let mut pos_with_question_marks = islands
                .iter()
                .flatten()
                .enumerate()
                .filter(|(_, c)| **c == '?')
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            // let mut pos_with_question_marks = Vec::new();
            // for (island_idx, island) in islands.iter().enumerate() {
            //     for (pos, &ch) in island.iter().enumerate() {
            //         if ch == '?' {
            //             pos_with_question_marks.push((island_idx, pos));
            //         }
            //     }
            // }

            println!("Pos with question marks: {:?}", pos_with_question_marks);

            let mut history: Vec<u32> = Vec::new();
            // let island_possibilities = transform(islands, pos_with_question_marks, points_to_add);
            let mut island_possibilities =
                get_variations(islands, pos_with_question_marks, records.len(), 0)
                    .iter()
                    .map(|x| {
                        x.iter()
                            .map(|island| {
                                let mut i = island.clone();
                                // i.sort();
                                i
                            })
                            .collect::<Vec<Vec<char>>>()
                    })
                    .collect::<Vec<Vec<Vec<char>>>>();

            island_possibilities.sort_by(|i1, i2| {
                for (island1, island2) in i1.iter().zip(i2.iter()) {
                    if island1.len() != island2.len() {
                        return island1.len().cmp(&island2.len());
                    }
                }
                return std::cmp::Ordering::Equal;
            });

            println!("Island possibilities:");
            for island in island_possibilities.clone() {
                for (x, i) in island.clone().iter().enumerate() {
                    println!("  {} - {:?}", x, i);
                }
                println!("-------");
            }
            for island in island_possibilities.clone() {
                if island.len() != records.len() {
                    continue;
                }
                let (combinations, next_history) =
                    get_islands_possibilities(&island, &records, &history);
                // for (i, r) in records.clone().iter().enumerate() {
                //     let empty = Vec::new();
                //     let mut history_for_record =
                //         history.get(*r as usize).unwrap_or(&empty).to_vec();
                //     history_for_record.push();
                //     history.insert(i, history_for_record.to_vec());
                // }
                if !next_history.is_empty() {
                    history = next_history;
                }
                total_combinations += combinations;
                if combinations > 0 {
                    println!(
                        "Adding combinations {} for possibility {:?}",
                        combinations, island
                    );
                }
            }
        } else {
            let islands_to_remove = island_diff;
            println!("Have to remove islands");
        }
        println!("------------------------------------------------");
        println!("Total combinations end: {}", total_combinations);
        println!("For line: {}", line);
        println!("------------------------------------------------");
        final_combinations += total_combinations;
    }

    // println!("Combination: {}", combinations);
    Ok(final_combinations)
}

pub fn part2(reader: BufReader<File>) -> io::Result<i32> {
    for line in reader.lines() {}
    Ok(1)
}

pub fn solution() {
    let path = Path::new("src/l_merry/input.txt");
    // let path = Path::new("src/l_merry/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 12, part 1 {}", part1.unwrap());
    println!("Day 12, part 2 {}", part2.unwrap());
}
