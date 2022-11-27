use std::collections::HashMap;
use std::fs;
use std::option::Option;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn part1(data: &Vec<String>) -> usize {
    let mut bags: HashMap<String, Bag> = HashMap::new();
    for line in data {
        let split: Vec<String> = line.split(" contain ").map(|x| x.to_string()).collect();
        let col: &str = split[0].strip_suffix(" bags").unwrap();
        let mut c_bag: Bag = Bag::new();
        c_bag.add_content(split[1].clone());
        bags.entry(col.to_string())
            .and_modify(|x| x.add_content(split[1].clone()))
            .or_insert(c_bag.clone());
        for (color, _num) in c_bag.contains.iter() {
            if bags.contains_key(color) {
                bags.entry(color.to_string())
                    .and_modify(|x| x.add_contained_by(col.to_string()));
            } else {
                let mut bag_t: Bag = Bag::new();
                bag_t.add_contained_by(col.to_string());
                bags.insert(color.to_string(), bag_t);
            }
        }
    }

    return get_depth(bags, "shiny gold".to_string());
}

fn get_depth(mut map: HashMap<String, Bag>, key: String) -> usize {
    let mut curr_key: Option<String> = Some(key);
    let mut keys: Vec<String> = Vec::new();
    let mut outer_bags: Vec<String> = Vec::new();
    while curr_key != None {
        let parents = &map.get_mut(&curr_key.unwrap()).unwrap().contained_by;
        for col in parents {
            if !outer_bags.contains(col) {
                outer_bags.push(col.to_string());
            }
        }
        keys.append(&mut parents.clone());
        curr_key = keys.pop();
    }

    return outer_bags.len();
}

fn part2(data: &Vec<String>) -> usize {
    let mut bags: HashMap<String, Bag> = HashMap::new();
    for line in data {
        let split: Vec<String> = line.split(" contain ").map(|x| x.to_string()).collect();
        let col: &str = split[0].strip_suffix(" bags").unwrap();
        let mut c_bag: Bag = Bag::new();
        c_bag.add_content(split[1].clone());
        bags.entry(col.to_string())
            .and_modify(|x| x.add_content(split[1].clone()))
            .or_insert(c_bag.clone());
        for (color, _num) in c_bag.contains.iter() {
            if bags.contains_key(color) {
                bags.entry(color.to_string())
                    .and_modify(|x| x.add_contained_by(col.to_string()));
            } else {
                let mut bag_t: Bag = Bag::new();
                bag_t.add_contained_by(col.to_string());
                bags.insert(color.to_string(), bag_t);
            }
        }
    }

    return count_contents(bags, "shiny gold".to_string());
}

fn count_contents(mut map: HashMap<String, Bag>, key: String) -> usize {
    let mut count: usize = 0;
    let mut curr_key: Option<String> = Some(key);
    let mut keys: Vec<String> = Vec::new();
    while curr_key != None {
        let contents = &map.get_mut(&curr_key.unwrap()).unwrap().contains;
        for (col, num) in contents.iter() {
            count += num;
            for _ in 0..*num { // Ugly but works lol
                keys.push(col.to_string());
            }
        }
        curr_key = keys.pop();
    }

    return count;
}

#[derive(Debug, Clone)]
struct Bag {
    contained_by: Vec<String>,
    contains: HashMap<String, usize>,
}

impl Bag {
    pub fn new() -> Self {
        Self {
            contained_by: Vec::new(),
            contains: HashMap::new(),
        }
    }
    pub fn add_content(&mut self, cont_str: String) {
        let contents: Vec<String> = cont_str
            .split(", ")
            .map(|x| {
                x.replace(" bags", "")
                    .replace(" bag", "")
                    .replace(".", "")
                    .trim()
                    .to_string()
            })
            .collect();
        for bag in contents {
            if bag.starts_with("no") {
                continue;
            }
            let tmp: Vec<&str> = bag.splitn(2, ' ').map(|x| x.trim()).collect();
            self.contains
                .insert(tmp[1].to_string(), tmp[0].parse::<usize>().unwrap());
        }
    }
    pub fn add_contained_by(&mut self, cont: String) {
        if !self.contained_by.contains(&cont) {
            self.contained_by.push(cont);
        }
    }
}
