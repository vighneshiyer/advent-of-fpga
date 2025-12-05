use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
struct Turn {
    dir: Direction,
    ticks: u32,
}

struct TurnResult {
    traversals_thru_zero: u32,
    dial: i32,
}

fn do_turn(turn: &Turn, dial: i32) -> TurnResult {
    let mut next_dial;
    let mut traversals_thru_zero;
    match turn {
        Turn {
            dir: Direction::Right,
            ticks,
        } => {
            next_dial = dial + *ticks as i32;
            traversals_thru_zero = (next_dial / 100) as u32;
        }
        Turn {
            dir: Direction::Left,
            ticks,
        } => {
            next_dial = dial - *ticks as i32;
            if next_dial <= 0 {
                traversals_thru_zero = 1 + (next_dial.unsigned_abs() / 100);
                // Don't double count the 0 position we started on
                if dial == 0 {
                    traversals_thru_zero -= 1;
                }
            } else {
                traversals_thru_zero = 0;
            }
        }
    }

    if next_dial > 0 {
        next_dial %= 100;
    } else if next_dial < 0 {
        next_dial = 100 - (next_dial.abs() % 100);
        // Edge case
        if next_dial == 100 {
            next_dial = 0;
        }
    }

    println!(
        "turn: {turn:?}, dial: {dial} -> {next_dial}, traversals_thru_zero: {traversals_thru_zero}"
    );
    assert!((0..=99).contains(&next_dial));

    TurnResult {
        traversals_thru_zero,
        dial: next_dial,
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input_file = args.get(1).expect("Provide a file as input!");
    let input_file_handle = File::open(Path::new(input_file))?;
    let reader = BufReader::new(input_file_handle);
    let mut turns = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let line_chars = line.chars().collect::<Vec<char>>();
        let (direction, ticks) = line_chars.split_first().expect("Empty string!");
        let direction = match direction {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Malformed direction"),
        };
        let ticks = ticks
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .expect("Malformed tick count");
        turns.push(Turn {
            dir: direction,
            ticks,
        });
    }

    let mut zeros_at_end_of_rotation: u32 = 0; // Only count zeros once the turn is done
    let mut zeros_during_rotation: u32 = 0; // Count zeros both when turn is done and during the turn
    let mut dial: i32 = 50;
    for turn in turns.iter() {
        let result = do_turn(turn, dial);
        dial = result.dial;
        zeros_at_end_of_rotation += (dial == 0) as u32;
        zeros_during_rotation += result.traversals_thru_zero;
    }
    println!("Zeros at end of rotation: {zeros_at_end_of_rotation}");
    println!("Zeros during and at end of rotation: {zeros_during_rotation}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn() {
        let r = do_turn(
            &Turn {
                dir: Direction::Right,
                ticks: 1000,
            },
            50,
        );
        assert_eq!(r.dial, 50);
        assert_eq!(r.traversals_thru_zero, 10);

        let r = do_turn(
            &Turn {
                dir: Direction::Left,
                ticks: 250,
            },
            51,
        );
        assert_eq!(r.dial, 1);
        assert_eq!(r.traversals_thru_zero, 2);
    }
}
