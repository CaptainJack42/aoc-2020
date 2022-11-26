use std::fs;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    // let data: Vec<String> = input.lines().into_iter().map(|x| x.to_string()).collect();
    let data: Vec<String> = input.split("\n\n").into_iter().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(data.clone()));
    println!("Part2: {}", part2(data));
}

fn part1(mut data: Vec<String>) -> usize {
    let req_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valids: usize = 0;
    for pp in data.iter_mut() {
        let fields: Vec<&str> = pp.split(|c| c == ':' || c == ' ' || c == '\n').collect();
        if req_fields.iter().all(|item| fields.contains(item)) {
            valids += 1;
        }
    }
    return valids;
}

fn part2(mut data: Vec<String>) -> usize {
    let req_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valids: usize = data.len() - 1;
    for pp in data.iter_mut() {
        let mut fields: Vec<&str> = pp.split(|c| c == ':' || c == ' ' || c == '\n').collect();
        if req_fields.iter().all(|item| fields.contains(item)) {
            fields = pp.split(|c| c == ' ' || c == '\n').collect();
            for field in fields.iter_mut() {
                let vals: Vec<&str> = field.split(':').collect();
                match vals[0] {
                    "byr" => {
                        if vals[1].parse::<usize>().unwrap_or(0) > 2002 || vals[1].parse::<usize>().unwrap_or(0) < 1920 {
                            valids -= 1;
                            break;
                        }
                    }
                    "iyr" => {
                        if vals[1].parse::<usize>().unwrap_or(0) > 2020 || vals[1].parse::<usize>().unwrap_or(0) < 2010 {
                            valids -= 1;
                            break;
                        }
                    }
                    "eyr" => {
                        if vals[1].parse::<usize>().unwrap_or(0) > 2030 || vals[1].parse::<usize>().unwrap_or(0) < 2020 {
                            valids -= 1;
                            break;
                        }
                    }
                    "hgt" => {
                        if vals[1].ends_with("cm") {
                            let height = vals[1].strip_suffix("cm").unwrap();
                            if height.parse::<usize>().unwrap_or(0) > 193 || height.parse::<usize>().unwrap_or(0) < 150 {
                                valids -= 1;
                                break;
                            }
                        } else if vals[1].ends_with("in") {
                            let height = vals[1].strip_suffix("in").unwrap();
                            if height.parse::<usize>().unwrap_or(0) > 76 || height.parse::<usize>().unwrap_or(0) < 59 {
                                valids -= 1;
                                break;
                            }
                        }
                    }
                    "hcl" => {
                        if !vals[1].starts_with('#') || vals[1].len() != 7 {
                            valids -=1;
                            break;
                        } else if !vals[1].strip_prefix('#').unwrap().chars().into_iter().all(|item| "abcdef0123456789".contains(item)) {
                            valids -=1;
                            break;
                        }
                    }
                    "ecl" => {
                        let ecl_vals: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                        if !ecl_vals.iter().any(|item| &vals[1] == item) {
                            valids -= 1;
                            break;
                        }
                    }
                    "pid" => {
                        if vals[1].len() != 9 || vals[1].parse::<usize>().unwrap_or(0) == 0 {
                            valids -= 1;
                            break;
                        }
                    }
                    &_ => continue,
                }
            }
        } else {
            valids -= 1;
            continue;
        }
    }
    
    return valids;
}

