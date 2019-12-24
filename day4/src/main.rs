use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start: i32 = args.get(1).and_then(|x| x.parse().ok()).expect("provide start");
    let end: i32 = args.get(2).and_then(|x| x.parse().ok()).expect("provide end");
    let total: usize = (start..end).into_iter().filter(|x| is_valid(*x)).count();

    println!("{}", total);
}

fn is_valid(pass: i32) -> bool {
    let mut rem = pass;
    let mut has_adjacent_2 = false;
    let mut current_run = 1;
    let mut prev_digit = 10;
    while rem > 0 {
        let last = rem % 10;
        if last > prev_digit {
            return false;
        }
        if last == prev_digit {
            current_run += 1;
        } else {
            has_adjacent_2 |= current_run == 2;
            current_run = 1;
        }
        prev_digit = last;
        rem /= 10;
    }
    return has_adjacent_2 || current_run == 2;
}
