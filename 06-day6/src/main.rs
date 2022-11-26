use std::fs;
use std::collections::HashMap;


fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<String> = input.split("\n\n").into_iter().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn part1(data: &Vec<String>) -> usize {
    let mut ans: usize = 0;
    for group in data {
        let mut questions: Vec<char> = Vec::new();
        for person in group.lines() {
            for letter in person.chars() {
                if !questions.contains(&letter) {
                    questions.push(letter);
                }
            }
        }
        ans += questions.len();
    }

    return ans;
}

fn part2(data: &Vec<String>) -> usize {
    let mut ans: usize = 0;
    for group in data {
        let mut questions: HashMap<char, usize> = HashMap::new();
        let mut c_persons: usize = 0;
        for person in group.lines() {
            for letter in person.chars() {
                questions.entry(letter).and_modify(|x| *x += 1).or_insert(1);
            }
            c_persons += 1;
        }
        for (_letter, count) in questions.into_iter() {
            if count == c_persons {
                ans += 1;
            }
        }
    }

    return ans;
}
