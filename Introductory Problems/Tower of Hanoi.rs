use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    println!("{}", (2_i32).pow(n) - 1);
    hanio(1, 2, 3, n);
}

fn hanio(a: u32, b: u32, c: u32, n: u32) {
    if n == 1 {
        println!("{} {}", a, c)
    } else {
        hanio(a, c, b, n - 1);
        hanio(a, b, c, 1);
        hanio(b, a, c, n - 1);
    }
}
