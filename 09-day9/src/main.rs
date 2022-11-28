use std::fs;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    let weak_no = part1(&data);
    println!("Part1: {}", weak_no);
    println!("Part2: {}", part2(&data, weak_no));
}

fn part1(data: &Vec<usize>) -> usize {
    for (idx, num) in data[25..].iter().enumerate() {
        if !data[idx..idx + 25].iter().any(|x| {
            data[idx..idx + 25]
                .iter()
                .any(|y| x + y == *num && x != y)
        }) {
            println!("idx: {}, num: {}", idx, *num);
            return *num;
        }
    }

    return 0;
}

fn part2(data: &Vec<usize>, weak_no: usize) -> usize {
    for (idx, _num) in data.iter().enumerate() {
        let mut sum = 0;
        let mut used: Vec<usize> = Vec::new();
        for n in data[idx..].iter() {
            sum += n;
            used.push(*n);
            if sum == weak_no {
                return used.iter().min().unwrap() + used.iter().max().unwrap();
            }
        }
    }
    return 0;
}
