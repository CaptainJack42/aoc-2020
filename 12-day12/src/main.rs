use std::fs;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn part1(data: &Vec<String>) -> isize {
    let mut ship: Ship = Ship::new();
    for line in data {
        ship.advance(line);
    }
    return ship.n_pos.abs() + ship.e_pos.abs();
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rotation {
    East = 0,
    South = 90,
    West = 180,
    North = 270,
}

impl Rotation {
    fn rotate(self, direction: &str, degrees: isize) -> Rotation {
        match direction {
            "L" => Rotation::from_isize((self as isize + (360 - degrees % 360)).abs() % 360),
            "R" => Rotation::from_isize((self as isize + degrees).abs() % 360),
            _ => panic!("Unknown rotation {}", direction),
        }
    }

    fn from_isize(value: isize) -> Rotation {
        match value {
            0 => East,
            90 => South,
            180 => West,
            270 => North,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

use Rotation::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Ship {
    rot: Rotation,
    n_pos: isize,
    e_pos: isize,
}

impl Ship {
    fn new() -> Ship {
        Self {
            rot: East,
            n_pos: 0,
            e_pos: 0,
        }
    }

    fn advance(&mut self, instr: &str) {
        let (action, value) = instr.split_at(1);
        match action {
            "L" => self.rot = self.rot.rotate(action, value.parse::<isize>().unwrap()),
            "R" => self.rot = self.rot.rotate(action, value.parse::<isize>().unwrap()),
            "F" => self.forward(value.parse::<isize>().unwrap()),
            "E" => self.e_pos += value.parse::<isize>().unwrap(),
            "S" => self.n_pos -= value.parse::<isize>().unwrap(),
            "W" => self.e_pos -= value.parse::<isize>().unwrap(),
            "N" => self.n_pos += value.parse::<isize>().unwrap(),
            &_ => panic!("Unknown action: {}", action),
        }
    }

    fn forward(&mut self, units: isize) {
        match self.rot {
            East => self.e_pos += units,
            South => self.n_pos -= units,
            West => self.e_pos -= units,
            North => self.n_pos += units,
        }
    }
}

fn part2(data: &Vec<String>) -> isize {
    let mut ship: Part2Ship = Part2Ship::new();
    for line in data {
        ship.advance(line);
    }
    return ship.n_pos.abs() + ship.e_pos.abs();
}

struct Waypoint {
    e_pos: isize,
    n_pos: isize,
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint {
            e_pos: 10,
            n_pos: 1,
        }
    }

    fn rotate_left(&mut self, degrees: isize) {
        for _ in 0..degrees / 90 {
            let tmp = self.n_pos;
            self.n_pos = self.e_pos;
            self.e_pos = tmp * (-1);
        }
    }

    fn rotate_right(&mut self, degrees: isize) {
        for _ in 0..degrees / 90 {
            let tmp = self.n_pos;
            self.n_pos = self.e_pos * (-1);
            self.e_pos = tmp;
        }
    }

    fn wp_move(&mut self, direction: &str, value: isize) {
        match direction {
            "N" => self.n_pos += value,
            "E" => self.e_pos += value,
            "S" => self.n_pos -= value,
            "W" => self.e_pos -= value,
            &_ => panic!("Unknown movement: {}", direction),
        }
    }
}

struct Part2Ship {
    e_pos: isize,
    n_pos: isize,
    waypoint: Waypoint,
}

impl Part2Ship {
    fn new() -> Part2Ship {
        Part2Ship {
            e_pos: 0,
            n_pos: 0,
            waypoint: Waypoint::new(),
        }
    }

    fn advance(&mut self, instruction: &str) {
        let (action, value) = instruction.split_at(1);
        match action {
            "N" => self.waypoint.wp_move(action, value.parse::<isize>().unwrap()),
            "S" => self.waypoint.wp_move(action, value.parse::<isize>().unwrap()),
            "E" => self.waypoint.wp_move(action, value.parse::<isize>().unwrap()),
            "W" => self.waypoint.wp_move(action, value.parse::<isize>().unwrap()),
            "L" => self.waypoint.rotate_left(value.parse::<isize>().unwrap()),
            "R" => self.waypoint.rotate_right(value.parse::<isize>().unwrap()),
            "F" => self.move_fw(value.parse::<isize>().unwrap()),
            &_ => panic!("Unknown action: {}", action),
        }
    }

    fn move_fw(&mut self, value: isize) {
        self.e_pos += self.waypoint.e_pos * value;
        self.n_pos += self.waypoint.n_pos * value;
    }
}
