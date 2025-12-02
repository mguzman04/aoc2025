enum Direction {
    Left,
    Right,
}

fn move_dial(initial_position: i32, direction: Direction, clicks: i32) -> i32 {
    match direction {
        Direction::Left => {
            // go negative
            if initial_position == 0 {
                return 100 - clicks;
            } else {
                let remainder = initial_position - clicks;
                remainder
            }
        }
        Direction::Right => {
            // go positive
            if initial_position == 99 {
                return 0 + clicks - 1;
            } else {
                0
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
}
