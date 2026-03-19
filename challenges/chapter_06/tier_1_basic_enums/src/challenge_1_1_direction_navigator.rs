// Challenge 1.1 - Direction Navigator
//
// Define `Direction` with variants:
// - North
// - East
// - South
// - West
//
// Then implement:
// - `opposite(dir: &Direction) -> Direction`
// - `turn_right(dir: &Direction) -> Direction`
// - `turn_left(dir: &Direction) -> Direction`
// - `navigation_sequence() -> Vec<Direction>` returning:
//   Start, Right, Right, Left, Right, Opposite

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn opposite(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

pub fn turn_right(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
    }
}

pub fn turn_left(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::West,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
        Direction::West => Direction::South,
    }
}

pub fn navigation_sequence() -> Vec<Direction> {
    let mut nsq: Vec<Direction> = Vec::new();
    let mut now = Direction::North;
    nsq.push(now);
    now = turn_right(&now);
    nsq.push(now);
    now = turn_right(&now);
    nsq.push(now);
    now = turn_left(&now);
    nsq.push(now);
    now = turn_right(&now);
    nsq.push(now);
    now = opposite(&now);
    nsq.push(now);
    nsq
}

// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .

#[cfg(test)]
mod tests {
    use super::{Direction, navigation_sequence, opposite, turn_left, turn_right};

    #[test]
    fn turn_functions_cover_all_four_directions() {
        assert_eq!(
            opposite(&Direction::North),
            Direction::South,
            "Opposite of North should be South."
        );
        assert_eq!(
            opposite(&Direction::West),
            Direction::East,
            "Opposite of West should be East."
        );
        assert_eq!(
            turn_right(&Direction::North),
            Direction::East,
            "Turning right from North should face East."
        );
        assert_eq!(
            turn_right(&Direction::West),
            Direction::North,
            "Turning right from West should wrap around to North."
        );
        assert_eq!(
            turn_left(&Direction::North),
            Direction::West,
            "Turning left from North should face West."
        );
        assert_eq!(
            turn_left(&Direction::East),
            Direction::North,
            "Turning left from East should face North."
        );
    }

    #[test]
    fn prompt_navigation_sequence_is_correct() {
        let expected = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::East,
            Direction::South,
            Direction::North,
        ];
        let actual = navigation_sequence();

        assert_eq!(
            actual, expected,
            "Navigation sequence should be Start=North, then East, South, East, South, North. Got {:?}.",
            actual
        );
    }
}
