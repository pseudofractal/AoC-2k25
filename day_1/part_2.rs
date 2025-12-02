use std::error::Error;
use std::fs;
use std::num::ParseIntError;

fn parse_rotation(line: &str) -> Result<(char, i32), ParseIntError> {
    let line = line.trim();
    let direction = line.chars().next().unwrap();
    let distance = line[1..].parse()?;
    Ok((direction, distance))
}

fn hits_during_rotation(start: i32, direction: char, distance: i32) -> i32 {
    let first_hit = match direction {
        'R' => {
            let offset = (100 - start) % 100;
            if offset == 0 {
                100
            } else {
                offset
            }
        }
        'L' => {
            if start == 0 {
                100
            } else {
                start
            }
        }
        _ => 0,
    };
    if distance < first_hit {
        0
    } else {
        1 + (distance - first_hit) / 100
    }
}

fn apply_rotation(current: i32, direction: char, distance: i32) -> i32 {
    match direction {
        'R' => (current + distance).rem_euclid(100),
        'L' => (current - distance).rem_euclid(100),
        _ => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut current_position = 50;
    let mut total_zero_hits = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (direction, distance) = parse_rotation(line)?;
        total_zero_hits += hits_during_rotation(current_position, direction, distance);
        current_position = apply_rotation(current_position, direction, distance);
    }
    println!("{}", total_zero_hits);
    Ok(())
}
