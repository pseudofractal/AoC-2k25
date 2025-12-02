use std::fs;
use std::error::Error;
use std::num::ParseIntError;

fn parse_rotation(line: &str) -> Result<(char, i32), ParseIntError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(('R', 0));
    }
    let direction = trimmed.chars().next().unwrap();
    let distance = trimmed[1..].parse()?;
    Ok((direction, distance))
}

fn apply_rotation(current_position: i32, direction: char, distance: i32) -> i32 {
    let delta = match direction {
        'R' => distance,
        'L' => -distance,
        _ => 0,
    };
    (delta + current_position).rem_euclid(100)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut current_position = 50;
    let mut num_zeroes = 0;
    for line in input.lines() {
        let (direction, distance) = parse_rotation(line)?;
        current_position = apply_rotation(current_position, direction, distance);
        if current_position == 0 {
            num_zeroes += 1;
        }
    }
    println!("Number of zeroes: {}", num_zeroes);
    Ok(())
}
