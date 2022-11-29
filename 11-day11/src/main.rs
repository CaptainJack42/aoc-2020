use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let map = gen_map(&data);
    println!("Part1: {}", part1(map.clone()));
    println!("Part2: {}", part2(map.clone()));
}

fn part1(mut map: HashMap<Coordinate, char>) -> usize {
    let mut stopped: bool = false;
    while !stopped {
        stopped = true;
        let prev_map = map.clone();
        for (coord, ch) in prev_map.iter() {
            if *ch == '.' {
                continue;
            }
            let mut occ_ne: usize = 0;
            for ne in get_neighboors(coord) {
                if prev_map.get(&ne).unwrap_or(&'.') == &'#' {
                    occ_ne += 1;
                }
            }
            if prev_map.get(coord).unwrap() == &'#' && occ_ne >= 4 {
                map.entry(*coord).and_modify(|e| *e = 'L');
                stopped = false;
            }
            if prev_map.get(coord).unwrap() == &'L' && occ_ne == 0 {
                map.entry(*coord).and_modify(|e| *e = '#');
                stopped = false;
            }
        }
    }

    return map.values().filter(|c| **c == '#').count();
}

fn get_neighboors(coord: &Coordinate) -> Vec<Coordinate> {
    let mut ret: Vec<Coordinate> = Vec::new();
    for x in coord.x - 1..coord.x + 2 {
        for y in coord.y - 1..coord.y + 2 {
            ret.push(Coordinate { x: x, y: y });
        }
    }
    ret.retain(|c| c != coord);
    return ret;
}

fn part2(mut map: HashMap<Coordinate, char>) -> usize {
    let mut stopped: bool = false;
    while !stopped {
        stopped = true;
        let prev_map = map.clone();
        for (coord, ch) in prev_map.iter() {
            if *ch == '.' {
                continue;
            }
            let mut occ_ne: usize = 0;
            for ne in get_visible_neighboors(coord, &map) {
                if prev_map.get(&ne).unwrap_or(&'.') == &'#' {
                    occ_ne += 1;
                }
            }
            if prev_map.get(coord).unwrap() == &'#' && occ_ne >= 5 {
                map.entry(*coord).and_modify(|e| *e = 'L');
                stopped = false;
            }
            if prev_map.get(coord).unwrap() == &'L' && occ_ne == 0 {
                map.entry(*coord).and_modify(|e| *e = '#');
                stopped = false;
            }
        }
    }

    return map.values().filter(|c| **c == '#').count();
}

fn get_visible_neighboors(coord: &Coordinate, map: &HashMap<Coordinate, char>) -> Vec<Coordinate> {
    let mut ret: Vec<Coordinate> = Vec::new();
    let delta: Vec<Coordinate> = vec![
        Coordinate { x: 0, y: 1 },
        Coordinate { x: 0, y: -1 },
        Coordinate { x: 1, y: 0 },
        Coordinate { x: 1, y: 1 },
        Coordinate { x: 1, y: -1 },
        Coordinate { x: -1, y: 0 },
        Coordinate { x: -1, y: 1 },
        Coordinate { x: -1, y: -1 },
    ];
    for c in delta {
        let mut range: isize = 1;
        loop {
            let curr: Coordinate = Coordinate {
                x: coord.x + c.x * range,
                y: coord.y + c.y * range,
            };
            if map.get(&curr).is_none() {
                break;
            } else if map.get(&curr).unwrap() == &'.' {
                range += 1;
                continue;
            }
            ret.push(curr);
            break;
        }
    }

    ret.retain(|c| c != coord);
    return ret;
}

fn gen_map(input: &Vec<String>) -> HashMap<Coordinate, char> {
    let mut map: HashMap<Coordinate, char> = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                Coordinate {
                    x: x as isize,
                    y: y as isize,
                },
                c,
            );
        }
    }
    return map;
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}
