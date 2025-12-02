use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;

fn parse_range(s: &str) -> Result<RangeInclusive<u128>, Box<dyn Error>> {
    let (left, right) = s
        .split_once('-')
        .ok_or_else(|| format!("range '{}' missing '-'", s))?;
    let start: u128 = left.trim().parse()?;
    let end: u128 = right.trim().parse()?;
    Ok(start..=end)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let ranges = input
        .trim()
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_range)
        .collect::<Result<Vec<_>, _>>()?;

    let mut sum: u128 = 0;

    for range in ranges {
        for num in range {
            let num_string = num.to_string();
            let mut idx: usize = 0;

            while idx < num_string.len() / 2 {
                let substring = &num_string[0..=idx];
                let num_occurrences = num_string.matches(substring).count();
                if (num_occurrences * substring.len()) == num_string.len() {
                    sum += num;
                    break;
                }
                idx += 1;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
