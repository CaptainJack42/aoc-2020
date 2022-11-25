use std::fs;


fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let mut data: Vec<usize> = input.lines().into_iter().map(|x| x.parse::<usize>().unwrap()).collect();
    data.sort_unstable();
    println!("Part1: {}", part1(&data));
    println!("Part1: {}", part2(&data));
}


fn part1(data: &Vec<usize>) -> usize {
    for x in data {
        if data.contains(&(2020 - x)) {
            return x * (2020 - x);
        }
    }
    return 0;
}

fn part2(data: &Vec<usize>) -> usize {
    for x in data {
        for y in data {
            if data.contains(&(2020 - x - y)){
                return x * y * (2020 - x - y);
            }
        }
    }
    return 0;
}

