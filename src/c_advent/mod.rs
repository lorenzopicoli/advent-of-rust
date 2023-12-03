use std::{
    cmp,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
    path::Path,
};
#[derive(Debug)]
struct AdventNumber {
    value: u32,
    row_position: usize,
    column_position: usize,
    length: usize,
}

// Could be a boolean matrix if part 2 is not needed
type SymPositions = Vec<Vec<Option<char>>>;

fn parse_lines(reader: BufReader<File>) -> Result<(Vec<AdventNumber>, SymPositions), io::Error> {
    let mut numbers = Vec::<AdventNumber>::new();
    let mut sym_positions: SymPositions = Vec::new();

    for (row, line) in reader.lines().enumerate() {
        let line = line?;
        let mut partial_number: String = "".to_string();
        let mut start_pos: Option<usize> = None;
        sym_positions.push(Vec::new());

        for (pos, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                partial_number.push(ch);
                if start_pos.is_none() {
                    start_pos = Some(pos);
                }
            } else {
                if let Some(start) = start_pos {
                    let number = AdventNumber {
                        value: partial_number.parse::<u32>().unwrap(),
                        row_position: row,
                        column_position: start,
                        length: (pos) - start,
                    };
                    numbers.push(number);
                }
                start_pos = None;
                partial_number = "".to_string();
            }

            if ch.is_ascii_punctuation() && ch != '.' {
                sym_positions[row].push(Some(ch));
            } else {
                sym_positions[row].push(None);
            }
        }

        // Handles the case where line ends with a number and we don't get a chance to insert the number
        // in the loop
        if let Some(start) = start_pos {
            let number = AdventNumber {
                value: partial_number.parse::<u32>().unwrap(),
                row_position: row,
                column_position: start,
                length: (line.chars().count()) - start,
            };
            numbers.push(number);
        }
        // println!("{}", line);
    }
    // println!("------");
    Ok((numbers, sym_positions))
}

fn is_engine_part_number(number: &AdventNumber, sym_positions: &SymPositions) -> bool {
    let safe_row_start = cmp::max((number.row_position as i32) - 1, 0) as usize;
    let safe_row_end = cmp::min(sym_positions.len(), number.row_position + 1);

    for row in sym_positions
        .iter()
        .take(safe_row_end + 1)
        .skip(safe_row_start)
    {
        let safe_col_start = cmp::max((number.column_position as i32) - 1, 0) as usize;
        let safe_col_end = cmp::min(row.len(), number.column_position + number.length + 1);
        for symbol in (*row).iter().take(safe_col_end).skip(safe_col_start) {
            if symbol.is_some() {
                // println!("Valid number {:#?}", number);
                return true;
            }
        }
    }
    false
}

fn range_intersects(range1: &Range<i32>, range2: &Range<i32>) -> bool {
    if range1.start <= range2.start && range1.end > range2.start {
        return true;
    }
    if range1.start < range2.end && range1.end >= range2.end {
        return true;
    }
    false
}

fn find_gear_ratio(sym_position: (i32, i32), numbers: &[AdventNumber]) -> u32 {
    let (sym_row, sym_col) = sym_position;
    let row_start = sym_row - 1;
    let row_end: i32 = sym_row + 1;
    let col_start = sym_col - 1;
    let col_end: i32 = sym_col + 1;

    let surrounding_row = row_start..row_end + 1;
    let surrounding_col = col_start..col_end + 1;

    let adjacent_numbers = numbers
        .iter()
        .filter(|number| {
            let number_col_range =
                (number.column_position as i32)..((number.column_position + number.length) as i32);
            surrounding_row.contains(&(number.row_position as i32))
                && range_intersects(&surrounding_col, &number_col_range)
        })
        .collect::<Vec<&AdventNumber>>();

    if adjacent_numbers.len() == 2 {
        adjacent_numbers[0].value * adjacent_numbers[1].value
    } else {
        0
    }
}

pub fn part1(reader: BufReader<File>) -> io::Result<u32> {
    let (numbers, sym_positions) = parse_lines(reader)?;
    // println!("{:#?}", sym_positions);
    // println!("{:#?}", numbers);
    let mut sum = 0;
    for number in numbers {
        if is_engine_part_number(&number, &sym_positions) {
            sum += number.value;
        }
    }
    Ok(sum)
}

pub fn part2(reader: BufReader<File>) -> io::Result<u32> {
    let (numbers, sym_positions) = parse_lines(reader)?;
    // println!("{:#?}", sym_positions);
    // println!("{:#?}", numbers);
    let mut sum = 0;
    for (row, sym_row) in sym_positions.iter().enumerate() {
        for (col, sym) in sym_row.iter().enumerate() {
            if let Some(ch) = sym {
                if *ch == '*' {
                    let gear_ratio = find_gear_ratio((row as i32, col as i32), &numbers);
                    sum += gear_ratio;
                }
            }
        }
    }
    Ok(sum)
}

pub fn solution() -> io::Result<u32> {
    let path = Path::new("src/c_advent/input.txt");
    // let path = Path::new("src/c_advent/example.txt");
    let mut file = File::open(path)?;
    let part1 = part1(io::BufReader::new(file));
    file = File::open(path)?;
    let part2 = part2(io::BufReader::new(file));

    println!("Problem 3, part 1 {:#?}", part1);

    part2
}
