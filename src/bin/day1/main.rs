#![allow(unused)]

use std::ops::AddAssign;

fn main() {
    let input = include_str!("input.txt");

    let mut dial = Dial::new();

    for line in input.trim().lines() {
        dial.turn(Turn::parse(line));
    }

    let times_stopped = dial.num_times_stopped_at_zero();
    let times_passed = dial.num_zeroes_seen();
    println!("Visited zero {times_stopped} times");
    println!("Passed zero {times_passed} times");
}

struct Dial {
    value: i16,
    num_times_stopped_at_zero: i16,
    num_zeroes_seen: i16,
}

impl Dial {
    fn new() -> Self {
        Self {
            value: 50,
            num_times_stopped_at_zero: 0,
            num_zeroes_seen: 0,
        }
    }

    fn current_value(&self) -> i16 {
        self.value
    }

    fn num_times_stopped_at_zero(&self) -> i16 {
        self.num_times_stopped_at_zero
    }

    fn num_zeroes_seen(&self) -> i16 {
        self.num_zeroes_seen
    }

    fn turn(&mut self, turn: Turn)  {
        let incr = match turn.direction {
            TurnDirection::Left => -1,
            TurnDirection::Right => 1,
        };

        let mut pointer = self.value;

        let mut num_distance_left = turn.distance;
        while num_distance_left > 0 {
            pointer += incr;

            if pointer == 100 {
                pointer = 0;
            } else if pointer == -1 {
                pointer = 99;
            }

            if pointer == 0 {
                self.num_zeroes_seen += 1;
            }

            num_distance_left -= 1;
        }

        if pointer == 0 {
            self.num_times_stopped_at_zero += 1;
        }

        self.value = pointer;
    }
}

#[cfg(test)]
mod test_dial {
    use crate::{Dial, Turn};

    #[test]
    fn test_initial_value() {
        let dial = Dial::new();

        assert_eq!(dial.current_value(), 50);
    }

    #[test]
    fn test_turn_left() {
        let mut dial = Dial::new();

        dial.turn(Turn::parse("L5"));

        assert_eq!(dial.current_value(), 45);
    }

    #[test]
    fn test_turn_right() {
        let mut dial = Dial::new();

        dial.turn(Turn::parse("R5"));

        assert_eq!(dial.current_value(), 55);
    }

    #[test]
    fn test_turn_above_max() {
        let mut dial = Dial::new();
        dial.turn(Turn::parse("R80"));

        assert_eq!(dial.current_value(), 30);
    }

    #[test]
    fn test_turn_below_min() {
        let mut dial = Dial::new();
        dial.turn(Turn::parse("L80"));

        assert_eq!(dial.current_value(), 70);
    }

    #[test]
    fn test_successive_turns() {
        let mut dial = Dial::new();

        dial.turn(Turn::parse("R30"));
        assert_eq!(dial.current_value(), 80);

        dial.turn(Turn::parse("R30"));
        assert_eq!(dial.current_value(), 10);

        dial.turn(Turn::parse("R30"));
        assert_eq!(dial.current_value(), 40);

        dial.turn(Turn::parse("R30"));
        assert_eq!(dial.current_value(), 70);
    }

    #[test]
    fn test_passing_zero() {
        let mut dial = Dial::new();
        dial.turn(Turn::parse("R100"));
        assert_eq!(dial.num_zeroes_seen(), 1);
        assert_eq!(dial.current_value(), 50);

        let mut dial = Dial::new();
        dial.turn(Turn::parse("R1000"));
        assert_eq!(dial.num_zeroes_seen(), 10);
        assert_eq!(dial.current_value(), 50);
    }

    #[test]
    fn test_adding_to_100() {
        let mut dial = Dial::new();
        dial.turn(Turn::parse("R50"));
        assert_eq!(dial.current_value(), 0);
        assert_eq!(dial.num_times_stopped_at_zero(), 1);
        assert_eq!(dial.num_zeroes_seen(), 1);
    }
}

#[derive(PartialEq, Debug)]
enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn parse(arg: &str) -> Self {
        match arg {
            "L" => TurnDirection::Left,
            "R" => TurnDirection::Right,
            _ => panic!("Unexpected turn direction"),
        }
    }
}

#[cfg(test)]
mod test_turn_direction {
    use crate::TurnDirection;

    #[test]
    fn test_parse_left() {
        let direction = TurnDirection::parse("L");

        assert_eq!(direction, TurnDirection::Left);
    }

    #[test]
    fn test_parse_right() {
        let direction = TurnDirection::parse("R");

        assert_eq!(direction, TurnDirection::Right);
    }
}

struct Turn {
    direction: TurnDirection,
    distance: i16,
}

impl Turn {
    pub fn parse(s: &str) -> Self {
        let (direction, distance) = s.split_at(1);

        Self {
            direction: TurnDirection::parse(direction),
            distance: i16::from_str_radix(distance, 10).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Turn, TurnDirection};

    #[test]
    fn test_parse_left() {
        let turn = Turn::parse("L16");

        assert_eq!(turn.direction, TurnDirection::Left);
    }

    #[test]
    fn test_parse_right() {
        let turn = Turn::parse("R16");

        assert_eq!(turn.direction, TurnDirection::Right);
    }

    #[test]
    fn test_parse_single_digit() {
        assert_eq!(Turn::parse("R1").distance, 1);
        assert_eq!(Turn::parse("R2").distance, 2);
        assert_eq!(Turn::parse("R3").distance, 3);
    }

    #[test]
    fn test_parse_double_digit() {
        let turn = Turn::parse("R98");

        assert_eq!(turn.distance, 98);
    }

    #[test]
    fn test_parse_triple_digit() {
        let turn = Turn::parse("R432");

        assert_eq!(turn.distance, 432);
    }
}
