use std::fs;

fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn part1(data: &Vec<String>) -> isize {
    let mut acc = 0;
    let mut curr_line = 0;
    let mut visited_lines: Vec<usize> = vec![0];
    loop {
        let instruction: Vec<&str> = data[curr_line].split(' ').collect();
        let op = instruction[0];
        let arg = instruction[1];
        match op {
            "acc" => {
                curr_line += 1;
                acc += arg.parse::<isize>().unwrap();
            }
            "jmp" => {
                curr_line = (curr_line as isize + arg.parse::<isize>().unwrap()) as usize;
            }
            "nop" => curr_line += 1,
            &_ => {}
        }
        if visited_lines.contains(&curr_line) {
            break;
        } else {
            visited_lines.push(curr_line);
        }
    }

    return acc;
}

fn part2(data: &Vec<String>) -> isize {
    let mut switch_instr = 0;
    loop {
        let mut acc = 0;
        let mut curr_line = 0;
        let mut visited_lines: Vec<usize> = vec![0];
        while data[switch_instr].starts_with("acc") || data[switch_instr] == "jmp +0".to_string() {
            switch_instr += 1;
        }
        loop {
            let instruction: Vec<&str> = data[curr_line].split(' ').collect();
            let op = if curr_line == switch_instr {
                if instruction[0] == "jmp" {
                    "nop"
                } else {
                    "jmp"
                }
            } else {
                instruction[0]
            };
            let arg = instruction[1];
            match op {
                "acc" => {
                    curr_line += 1;
                    acc += arg.parse::<isize>().unwrap();
                }
                "jmp" => {
                    curr_line = (curr_line as isize + arg.parse::<isize>().unwrap()) as usize;
                }
                "nop" => curr_line += 1,
                &_ => {}
            }
            if visited_lines.contains(&curr_line) {
                switch_instr += 1;
                break;
            } else if curr_line >= data.len() {
                return acc;
            } else {
                visited_lines.push(curr_line);
            }
        }
    }
}
