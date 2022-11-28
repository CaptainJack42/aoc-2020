use std::fs;
use std::collections::HashMap;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let mut data: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    println!("Part1: {}", part1(&mut data.clone()));
    println!("Part2: {}", part2(&mut data));
}

fn part1(data: &mut Vec<usize>) -> usize {
    data.sort_unstable();
    let mut last: usize = 0;
    let mut counter: HashMap<usize, usize> = HashMap::new();
    for e in data {
        counter.entry(*e - last).and_modify(|x| *x += 1).or_insert(1);
        last = *e;
    }
    return counter.get(&1).unwrap() * (counter.get(&3).unwrap() + 1);
}

fn part2(data: &mut Vec<usize>) -> usize {
    data.sort_unstable();
    data.push(data[data.len() - 1] + 3);
    let mut counter: HashMap<isize, usize> = HashMap::from([
        (0, 1),
    ]);
    for ad in data {
        counter.insert(*ad as isize,
            counter.get(&(*ad as isize - 3)).unwrap_or(&0) +
            counter.get(&(*ad as isize - 2)).unwrap_or(&0) +
            counter.get(&(*ad as isize - 1)).unwrap_or(&0)
        );
    }
    return *counter.get(counter.keys().max().unwrap()).unwrap();
}
