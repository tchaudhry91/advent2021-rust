use eyre::Result;
use std::fs;

fn main() {
    let depths = get_input_vec(String::from("input.txt")).expect("Could not read input file");
    println!("Total Increasing Depths:{}", get_ascending(&depths));
    println!(
        "Total Increasing Depths By 3 Position Sliding Window:{}",
        get_ascending_in_window(&depths, 3)
    );
}

fn get_input_vec(filename: String) -> Result<Vec<i32>> {
    let contents = fs::read_to_string(filename)?;
    let mut depths: Vec<i32> = vec![];
    for d in contents.split('\n') {
        if d.is_empty() {
            continue;
        }
        depths.push(d.parse::<i32>()?);
    }
    Ok(depths)
}

fn get_ascending(depths: &[i32]) -> i32 {
    let mut total: i32 = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            total += 1;
        }
    }
    total
}

fn get_ascending_in_window(depths: &[i32], window_size: usize) -> i32 {
    let mut total: i32 = 0;
    let mut previous_window_sum: i32 = 0;
    for i in 0..depths.len() - (window_size - 1) {
        let window = &depths[i..i + window_size];
        if i == 0 {
            previous_window_sum = window.iter().sum();
            continue;
        }
        if window.iter().sum::<i32>() > previous_window_sum {
            total += 1;
        }
        previous_window_sum = window.iter().sum();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ascending() {
        let depths = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 19, 21];
        assert_eq!(get_ascending(&depths), 8);
    }

    #[test]
    fn test_get_ascending_in_window() {
        let depths = vec![0, 1, 1, 2, 0, 2];
        assert_eq!(get_ascending_in_window(&depths, 3), 2);
    }
}
