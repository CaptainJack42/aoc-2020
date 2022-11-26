use std::fs;


fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<String> = input.lines().into_iter().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn part1(data: &Vec<String>) -> usize {
    let mut c_valids: usize = 0;
    for x in data {
        let mut y: Vec<&str> = x.split(':').collect();
        let rule: String = y[0].to_string();
        let pw: String = y[1].to_string();
        let letter: char = rule.chars().last().unwrap();
        y = rule.split(' ').collect();
        y = y[0].split('-').collect();
        let min: usize = y[0].parse::<usize>().unwrap();
        let max: usize = y[1].parse::<usize>().unwrap();
        if pw.matches(letter).count() >= min && pw.matches(letter).count() <= max {
            c_valids += 1;
        }
    }
    return c_valids;
}

fn part2(data: &Vec<String>) -> usize {
    let mut c_valids: usize = 0;
    for x in data {
        let mut y: Vec<&str> = x.split(':').collect();
        let rule: String = y[0].to_string();
        let pw: String = y[1].strip_prefix(' ').unwrap().to_string();
        let letter: char = rule.chars().last().unwrap();
        y = rule.split(' ').collect();
        y = y[0].split('-').collect();
        let first: usize = y[0].parse::<usize>().unwrap();
        let second: usize = y[1].parse::<usize>().unwrap();
        let pw_first = pw.chars().nth(first - 1).unwrap();
        let pw_sec = pw.chars().nth(second -1).unwrap();
        if (pw_first == letter && pw_sec != letter) || (pw_first != letter && pw_sec == letter) {
            c_valids += 1;
        }
    }
    return c_valids;
}
