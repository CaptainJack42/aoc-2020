use std::fs;
use std::default::Default;


fn main() {
    let path = "input";
    let input = fs::read_to_string(path).unwrap();
    let data: Vec<String> = input.lines().into_iter().map(|x| x.to_string()).collect();
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn part1(data: &Vec<String>) -> usize {
    let mut ans: usize = 0;
    let mut x: usize = 0;
    for line in data {
        if line.chars().nth(x % line.len()).unwrap() == '#' {
            ans += 1;
        }
        x += 3;
    }

    return ans;
}

fn part2(data: &Vec<String>) -> usize {
    let mut routes: Vec<Route> = vec![
        Route {
            right: 1,
            down: 1,
            ..Default::default()
        },
        Route {
            right: 3,
            down: 1,
            ..Default::default()
        },
        Route {
            right: 5,
            down: 1,
            ..Default::default()
        },
        Route {
            right: 7,
            down: 1,
            ..Default::default()
        },
        Route {
            right: 1,
            down: 2,
            ..Default::default()
        }
    ];
    for (y, line) in data.iter().enumerate() {
        for route in routes.iter_mut() {
            if y % route.down > 0 {
                continue;
            }
            if line.chars().nth(route.x_pos % line.len()).unwrap() == '#'{
                route.n_trees += 1;
            }
            route.x_pos += route.right;
        }
    }

    let mut ans: usize = 1;
    for route in routes {
        ans = ans * route.n_trees;
    }
    return ans;
}

#[derive(Debug, Default)]
struct Route {
    right: usize,
    down: usize,
    x_pos: usize,
    n_trees: usize,
}
