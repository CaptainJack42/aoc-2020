use std::fs;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let mut data: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(&data));
    // println!("Part2: {}", part2(&mut data));
}

fn part1(data: &Vec<String>) -> usize {
    return 0;
}
