use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut codes: Vec<usize> = input.trim().split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .clone();
    codes[1] = 12;
    codes[2] = 2;
    let mut cur: usize = 0;

    loop {
        let op = codes[cur];
        match op {
            1 => {
                let res = codes[codes[cur+1]]+codes[codes[cur+2]];
                let idx = codes[cur+3];
                codes[idx] = res;
            }
            2 => {
                let res = codes[codes[cur+1]]*codes[codes[cur+2]];
                let idx = codes[cur+3];
                codes[idx] = res;
            }
            99 => {}
            _ => panic!(),
        }
        if op == 99 {
            println!("{}", codes[0]);
            break;
        } else {
            cur += 4;
        }
    }
}
