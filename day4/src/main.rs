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
    let mut has_adjacent = false;
    let mut prev_digit = 10;
    while rem > 0 {
        let last = rem % 10;
        has_adjacent |= (last == prev_digit);
        if last > prev_digit {
            return false;
        }
        prev_digit = last;
        rem /= 10;
    }
    return has_adjacent;
}
