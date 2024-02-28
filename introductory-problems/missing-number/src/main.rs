use std::io;

pub fn main() {
    let mut n = String::new();
    let mut input = String::new();
    io::stdin().read_line(&mut n).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = n.trim().parse().unwrap();
    println!(
        "{}",
        ((1 + n) * n) / 2 -
            input
                .trim()
                .split_whitespace()
                .into_iter()
                .fold(0, |s, x| s + x.parse::<u64>().unwrap())
    );
}
