use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    println!("{}", power_mod(2, n, ((10_i64).pow(9) + 7) as u64));
}

pub fn power_mod(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut exponent = exponent;
    let mut base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent /= 2;
        base = (base * base) % modulus;
    }
    result
}
