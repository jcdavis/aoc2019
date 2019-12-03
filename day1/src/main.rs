use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let sum: i32 = input.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .map(|e| {
            let mut cur = e;
            let mut total = 0;
            while cur > 0 {
                cur = (cur/3) - 2;
                total += cur.max(0);
            }
            total
        })
        .sum();

    println!("input: {}", sum);
}
