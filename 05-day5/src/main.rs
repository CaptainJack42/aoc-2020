use std::fs;


fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<String> = input.lines().into_iter().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn part1(data: &Vec<String>) -> usize {
    let mut ans: usize = 0;
    for line in data {
        let (row_s, seat_s) = line.split_at(7);
        let row_s = row_s.replace("F", "0").replace("B", "1");
        let seat_s = seat_s.replace("L", "0").replace("R", "1");
        let id = usize::from_str_radix(&row_s, 2).unwrap() * 8 + usize::from_str_radix(&seat_s, 2).unwrap();
        ans = if id > ans { id } else { ans };
    }

    return ans;
}

fn part2(data: &Vec<String>) -> usize {
    let mut seat_ids: Vec<usize> = vec![];

    for line in data {
        let (row_s, seat_s) = line.split_at(7);
        let row_s = row_s.replace("F", "0").replace("B", "1");
        let seat_s = seat_s.replace("L", "0").replace("R", "1");
        let id: usize = usize::from_str_radix(&row_s, 2).unwrap() * 8 + usize::from_str_radix(&seat_s, 2).unwrap();
        seat_ids.push(id);
    }

    seat_ids.sort_unstable();

    for i in 1..seat_ids[seat_ids.len() - 1] {
        if seat_ids.contains(&i) { continue } else {
            if seat_ids.contains(&(i + 1)) && seat_ids.contains(&(i - 1)){
                return i;
            }
        }
    }
    return 0;
}

