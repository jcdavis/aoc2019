use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let fixed: Vec<usize> = input.trim().split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .clone();
    
    for noun in 0..100 {
        for verb in 0..100 {
            let mut codes = fixed.clone();
            codes[1] = noun;
            codes[2] = verb;
            if eval(&mut codes) == 19690720 {
                println!("{}", 100*noun+verb);
                break;
            }
        }
    }
   
}

fn eval(codes: &mut Vec<usize>) -> usize {
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
            return codes[0];
        } else {
            cur += 4;
        }
    }
}
