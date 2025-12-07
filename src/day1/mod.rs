use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(PartialEq)]
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
        let crossings: i32;
        (dial_position, crossings) = move_dial(dial_position, &rotation.direction, rotation.clicks);
        total_crossings += crossings;
        if dial_position == 0 {
            total_zeros += 1;
        }
    }
    (total_zeros, total_crossings)
}

/// Returns the new position of the dial
fn move_dial(initial_position: i32, direction: &Direction, clicks: i32) -> (i32, i32) {
    let total_movement = match direction {
        Direction::Left => -clicks,
        Direction::Right => clicks,
    };
    let gross_position = initial_position + total_movement;
    let final_position = (gross_position % 100 + 100) % 100;

    // Calculate Crossings
    let full_cycles = clicks / 100;
    let rem_clicks = clicks % 100;
    let mut total_crossings = full_cycles;

    match direction {
        Direction::Left => {
            if rem_clicks > 0 && initial_position > 0 && rem_clicks >= initial_position {
                total_crossings += 1;
            }
        }
        Direction::Right => {
            if rem_clicks > 0 && initial_position + rem_clicks > 99 {
                total_crossings += 1;
            }
        }
    }

    (final_position, total_crossings)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dial_left_from_zero() {
        let (new_position, total_crossings) = move_dial(0, &Direction::Left, 1);
        assert_eq!(
            new_position, 99,
            "Expected new position to be 99 not {}",
            new_position
        );
        assert_eq!(total_crossings, 0, "Expected not to cross zero");
    }

    #[test]
    fn dial_left_from_one() {
        let (new_position, total_crossings) = move_dial(1, &Direction::Left, 1);
        assert_eq!(
            new_position, 0,
            "Expected new position to be 0 not {}",
            new_position
        );
        assert_eq!(total_crossings, 1, "Expected to cross zero");
    }

    #[test]
    fn dial_left_middle_change() {
        let (new_position, total_crossings) = move_dial(50, &Direction::Left, 10);
        assert_eq!(
            new_position, 40,
            "Expected new position to be 40 not {}",
            new_position
        );
        assert_eq!(total_crossings, 0, "Expected not to cross zero");
    }

    #[test]
    fn dial_left_middle_change_cycle() {
        let (new_position, total_crossings) = move_dial(10, &Direction::Left, 20);
        assert_eq!(
            new_position, 90,
            "Expected new position to be 90 not {}",
            new_position
        );
        assert_eq!(total_crossings, 1, "Expected to cross zero");
    }

    #[test]
    fn land_on_zero_left() {
        let (new_position, total_crossings) = move_dial(10, &Direction::Left, 10);
        assert_eq!(
            new_position, 0,
            "Expected new position to be 0 not {}",
            new_position
        );
        assert_eq!(total_crossings, 1, "Expected to cross zero");
    }

    #[test]
    fn dial_left_multiple_cycles() {
        let (new_position, total_crossings) = move_dial(95, &Direction::Left, 200);
        assert_eq!(new_position, 95, "Expected to be on the same position");
        assert_eq!(total_crossings, 2, "Expected to cross zero");
    }

    #[test]
    fn dial_right_from_99() {
        let (new_position, total_crossings) = move_dial(99, &Direction::Right, 1);
        assert_eq!(
            new_position, 0,
            "Expected new position to be 0 not {}",
            new_position
        );
        assert_eq!(total_crossings, 1, "Expected to cross zero");
    }

    #[test]
    fn dial_right_middle_change() {
        let (new_position, total_crossings) = move_dial(40, &Direction::Right, 10);
        assert_eq!(
            new_position, 50,
            "Expected new position to be 50 not {}",
            new_position
        );
        assert_eq!(total_crossings, 0, "Expected not to cross zero");
    }

    #[test]
    fn dial_right_middle_change_cycle() {
        let (new_position, total_crossings) = move_dial(90, &Direction::Right, 20);
        assert_eq!(
            new_position, 10,
            "Expected new position to be 10 not {}",
            new_position
        );
        assert_eq!(total_crossings, 1, "Expected to cross zero");
    }

    #[test]
    fn land_on_zero_right() {
        let (new_position, total_crossings) = move_dial(90, &Direction::Right, 10);
        assert_eq!(
            new_position, 0,
            "Expected new position to be 0 not {}",
            new_position
        );
        assert_eq!(total_crossings, 1, "Expected to cross zero");
    }

    #[test]
    fn dial_right_multiple_cycles() {
        let (new_position, total_crossings) = move_dial(10, &Direction::Right, 200);
        assert_eq!(new_position, 10, "Expected to be on the same position");
        assert_eq!(total_crossings, 2, "Expected to cross zero");
    }
}
