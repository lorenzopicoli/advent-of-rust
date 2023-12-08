use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn get_strength_joker(cards: &Vec<u8>) -> u8 {
    let mut occurrences = [0; 15];
    for card in cards {
        occurrences[*card as usize] += 1;
    }
    // let mut high_card = 0;
    let mut pairs = 0;
    let mut three_of_a_kind = 0;
    let mut four_of_a_kind = 0;
    let mut five_of_a_kind = 0;
    let jokers = occurrences[1];
    // println!("cards {:?}", cards);
    // println!("jokers {}", jokers);
    // println!("occurrences {:?}", occurrences);
    for (i, occurrence) in occurrences.iter().enumerate() {
        if i == 1 {
            continue;
        }
        match occurrence {
            0 => (),
            // 1 => high_card += 1,
            2 => pairs += 1,
            3 => three_of_a_kind += 1,
            4 => four_of_a_kind += 1,
            5 => five_of_a_kind += 1,
            _ => (),
        };
    }
    if five_of_a_kind == 1
        || (four_of_a_kind == 1 && jokers == 1)
        || (three_of_a_kind == 1 && jokers == 2)
        || (pairs == 1 && jokers == 3)
        || (jokers >= 4)
    {
        return 6;
    }
    if four_of_a_kind == 1
        || (three_of_a_kind == 1 && jokers == 1)
        || (pairs == 1 && jokers == 2)
        || (jokers == 3)
    {
        return 5;
    }
    if (three_of_a_kind == 1 && pairs == 1)
        || (pairs == 2 && jokers == 1)
        || (jokers == 2 && pairs == 1)
    {
        return 4;
    }
    if three_of_a_kind == 1 || (pairs == 1 && jokers == 1) || (jokers == 2) {
        return 3;
    }
    if pairs == 2 || (pairs == 1 && jokers == 1) {
        return 2;
    }
    if pairs == 1 {
        return 1;
    }
    if jokers == 1 {
        return 1;
    }
    0
}

fn get_strength(cards: &Vec<u8>) -> u8 {
    let mut occurrences = [0; 15];
    for card in cards {
        occurrences[*card as usize] += 1;
    }
    // let mut high_card = 0;
    let mut pairs = 0;
    let mut three_of_a_kind = 0;
    let mut four_of_a_kind = 0;
    let mut five_of_a_kind = 0;
    for occurrence in occurrences {
        match occurrence {
            0 => (),
            // 1 => high_card += 1,
            2 => pairs += 1,
            3 => three_of_a_kind += 1,
            4 => four_of_a_kind += 1,
            5 => five_of_a_kind += 1,
            _ => (),
        };
    }
    if five_of_a_kind == 1 {
        return 6;
    }
    if four_of_a_kind == 1 {
        return 5;
    }
    if three_of_a_kind == 1 && pairs == 1 {
        return 4;
    }
    if three_of_a_kind == 1 {
        return 3;
    }
    if pairs == 2 {
        return 2;
    }
    if pairs == 1 {
        return 1;
    }
    0
}

fn split_str(value: &str, sub: &str) -> Vec<String> {
    let split = value.split(sub);
    split.map(|e| e.to_string()).collect::<Vec<String>>()
}

fn to_digit_chunks(num_str: &String) -> Vec<i32> {
    let chars: Vec<char> = num_str.chars().collect();

    chars
        .chunks(2)
        .map(|chunk| {
            let digit_str: String = chunk.iter().collect();
            digit_str.parse::<i32>().unwrap_or(0)
        })
        .collect()
}

pub fn part1(reader: BufReader<File>) -> io::Result<u32> {
    // (bid, strength, cards in order)
    let mut results: Vec<(u32, u8, String)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts = split_str(&line, " ");
        let cards_str = parts[0].clone();
        let bid = parts[1].clone();
        let mut cards: Vec<u8> = Vec::new();
        for char in cards_str.chars() {
            if char == 'A' {
                cards.push(14);
            } else if char == 'T' {
                cards.push(10);
            } else if char == 'J' {
                cards.push(11);
            } else if char == 'Q' {
                cards.push(12);
            } else if char == 'K' {
                cards.push(13);
            } else {
                cards.push(char.to_digit(10).unwrap() as u8);
            }
        }
        let strength = get_strength(&cards);
        results.push((
            bid.parse::<u32>().unwrap(),
            strength,
            cards
                .iter()
                .fold(String::new(), |acc, &x| acc + &format!("{:02}", x)),
        ));
    }
    results.sort_by(|a, b| {
        if b.1 == a.1 {
            let chunks1 = to_digit_chunks(&a.2);
            let chunks2 = to_digit_chunks(&b.2);
            for (chunk1, chunk2) in chunks1.iter().zip(chunks2.iter()) {
                if chunk1 != chunk2 {
                    return chunk2.cmp(chunk1);
                }
            }
        }
        b.1.cmp(&a.1)
    });
    let length = results.len();
    let mut sum = 0;
    for (i, r) in results.iter().enumerate() {
        sum += r.0 * (length - i) as u32;
    }
    Ok(sum)
}

pub fn part2(reader: BufReader<File>) -> io::Result<u32> {
    //  (bid, strength, cards in order)
    let mut results: Vec<(u32, u8, String)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts = split_str(&line, " ");
        let cards_str = parts[0].clone();
        let bid = parts[1].clone();
        let mut cards: Vec<u8> = Vec::new();
        for char in cards_str.chars() {
            if char == 'A' {
                cards.push(14);
            } else if char == 'T' {
                cards.push(10);
            } else if char == 'J' {
                cards.push(1);
            } else if char == 'Q' {
                cards.push(12);
            } else if char == 'K' {
                cards.push(13);
            } else {
                cards.push(char.to_digit(10).unwrap() as u8);
            }
        }
        let strength = get_strength_joker(&cards);
        results.push((
            bid.parse::<u32>().unwrap(),
            strength,
            cards
                .iter()
                .fold(String::new(), |acc, &x| acc + &format!("{:02}", x)),
        ));
    }
    results.sort_by(|a, b| {
        if b.1 == a.1 {
            let chunks1 = to_digit_chunks(&a.2);
            let chunks2 = to_digit_chunks(&b.2);
            for (chunk1, chunk2) in chunks1.iter().zip(chunks2.iter()) {
                if chunk1 != chunk2 {
                    return chunk2.cmp(chunk1);
                }
            }
        }
        b.1.cmp(&a.1)
    });
    let length = results.len();
    let mut sum = 0;
    for (i, r) in results.iter().enumerate() {
        sum += r.0 * (length - i) as u32;
    }
    Ok(sum)
}

pub fn solution() {
    let path = Path::new("src/g_I/input.txt");
    // let path = Path::new("src/g_I/example.txt");
    let mut file = File::open(path).unwrap();
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path).unwrap();
    let part2 = part2(io::BufReader::new(file));

    println!("Day 7, part 1 {}", part1.unwrap());
    println!("Day 7, part 2 {}", part2.unwrap());
}
