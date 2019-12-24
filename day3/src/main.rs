use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let mut grid: HashMap<(i32, i32), (usize, i32)> = HashMap::new();
    trace(&mut grid, &lines, 0);
    println!("{}", trace(&mut grid, &lines, 1));
}

fn trace(grid: &mut HashMap<(i32, i32), (usize, i32)>, lines: &Vec<String>, num: usize) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut min_dist = 1000000000;
    let mut steps = 0;
    for dir in lines[num].split(',') {
        let mut chars = dir.chars();
        let (dx, dy) = match chars.next().unwrap() {
            'U' => (0,1),
            'D' => (0,-1),
            'R' => (1,0),
            'L' => (-1,0),
            _ => panic!(),
        };
        let mut len: i32 = chars.as_str().parse().unwrap();

        while len > 0 {
            x += dx;
            y += dy;
            steps += 1;
            match grid.get(&(x,y)) {
                Some((n, s)) => {
                    if n != &num {
                        min_dist = min_dist.min(s + steps);
                    }
                },
                None => {
                    grid.insert((x,y), (num, steps));
                    ()
                },
            }
            len -= 1;
        }
    }
    min_dist
}
