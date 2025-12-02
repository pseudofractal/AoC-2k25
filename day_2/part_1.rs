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

fn decimal_digits(mut n: u128) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut digits = 0u32;
    while n != 0 {
        n /= 10;
        digits += 1;
    }
    digits
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
        for n in range {
            let digits = decimal_digits(n);
            if digits % 2 != 0 {
                continue;
            }

            let half = (digits / 2) as u32;
            let pow10 = match 10u128.checked_pow(half) {
                Some(p) => p,
                None => continue,
            };

            let first = n / pow10;
            let second = n % pow10;

            if first == second {
                sum = sum.saturating_add(n);
            }
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}
