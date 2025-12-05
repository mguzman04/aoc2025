use std::fs::File;
use std::io::{BufReader, Lines};

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    clicks: i32,
}

impl Rotation {
    fn new(input: String) -> Rotation {
        let (direction, clicks) = input.split_at(1);
        Rotation {
            direction: Self::to_direction(direction),
            clicks: clicks.parse::<i32>().expect("Failed to parse string"),
        }
    }

    fn to_direction(directtion_str: &str) -> Direction {
        if directtion_str == "L" {
            return Direction::Left;
        }
        Direction::Right
    }
}

/// Return a tuple where
/// the first digit is the number of zero landings
/// and the second digit is the number of zero crossings
pub fn count_zero_landings(lines: Lines<BufReader<File>>) -> (i32, i32) {
    let mut total_zeros = 0;
    let mut total_crossings = 0;
    let mut dial_position = 50;
    for line in lines.map_while(Result::ok) {
        let rotation = Rotation::new(line);
        dial_position = move_dial(dial_position, rotation.direction, rotation.clicks);
        if dial_position == 0 {
            total_zeros += 1;
        }
        if dial_position == 0 || zero_crossing(dial_position, rotation.clicks) {
            total_crossings += 1;
        }
    }
    (total_zeros, total_crossings)
}

/// Returns True if dial crossed zero
fn zero_crossing(initial_position: i32, clicks: i32) -> bool {
    let mut count = clicks / 100;
    if clicks % 100 > initial_position {
        count += 1;
    }
    count > 0
}

/// Returns the new position of the dial
fn move_dial(initial_position: i32, direction: Direction, clicks: i32) -> i32 {
    let clicks = clicks % 100;
    match direction {
        Direction::Left => {
            // go negative
            if initial_position == 0 {
                return 100 - clicks;
            } else {
                let remainder = initial_position - clicks;
                if remainder < 0 {
                    return 100 + remainder;
                }
                remainder
            }
        }
        Direction::Right => {
            // go positive
            if initial_position == 99 {
                return clicks - 1;
            } else {
                let sum = initial_position + clicks;
                if sum > 99 {
                    return sum - 100;
                }
                sum
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dial_left_from_zero() {
        let new_position = move_dial(0, Direction::Left, 1);
        assert_eq!(
            new_position, 99,
            "Expected new position to be 99 not {}",
            new_position
        );
    }

    #[test]
    fn dial_left_middle_change() {
        let new_position = move_dial(50, Direction::Left, 10);
        assert_eq!(
            new_position, 40,
            "Expected new position to be 40 not {}",
            new_position
        );
    }

    #[test]
    fn dial_left_middle_change_cycle() {
        let new_position = move_dial(10, Direction::Left, 20);
        assert_eq!(
            new_position, 90,
            "Expected new position to be 90 not {}",
            new_position
        );
    }

    #[test]
    fn dial_left_multiple_cycles() {
        let new_position = move_dial(95, Direction::Left, 200);
        assert_eq!(new_position, 95, "Expected to be on the same position");
    }

    #[test]
    fn dial_right_from_99() {
        let new_position = move_dial(99, Direction::Right, 1);
        assert_eq!(
            new_position, 0,
            "Expected new position to be 0 not {}",
            new_position
        );
    }

    #[test]
    fn dial_right_middle_change() {
        let new_position = move_dial(40, Direction::Right, 10);
        assert_eq!(
            new_position, 50,
            "Expected new position to be 50 not {}",
            new_position
        );
    }

    #[test]
    fn dial_right_middle_change_cycle() {
        let new_position = move_dial(90, Direction::Right, 20);
        assert_eq!(
            new_position, 10,
            "Expected new position to be 10 not {}",
            new_position
        );
    }

    #[test]
    fn dial_right_multiple_cycles() {
        let new_position = move_dial(10, Direction::Right, 200);
        assert_eq!(new_position, 10, "Expected to be on the same position");
    }

    #[test]
    fn zero_crossing_none() {
        let did_cross = zero_crossing(10, 5);
        assert!(!did_cross, "Expected not to cross zero");
    }

    #[test]
    fn zero_crossing_multiple_cycles() {
        let did_cross = zero_crossing(10, 300);
        assert!(did_cross, "Expected to cycle 3 times");
    }
}
