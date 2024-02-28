use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    println!("{}", trailing_zeros(n));
}

fn trailing_zeros(n: u64) -> u64 {
    let mut count = 0;
    let mut n = n;
    while n > 0 {
        count += n / 5;
        n /= 5;
    }
    count
}
